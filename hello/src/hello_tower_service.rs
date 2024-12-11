#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fmt::Display, future::Future, pin::Pin, time::Duration};

struct Server {}

struct Request {}

#[derive(Debug)]
struct Response {
    code: usize,
}

trait SetHeader {
    fn set_header(&self, header_key: &str, header_value: &str);
}

impl Response {
    fn ok() -> Self {
        Response { code: 200 }
    }
    fn not_found() -> Self {
        Response { code: 404 }
    }
}

impl SetHeader for Response {
    fn set_header(&self, header_key: &str, header_value: &str) {}
}

type Result<T> = core::result::Result<T, anyhow::Error>;

async fn read_request() -> Result<Request> {
    Ok(Request {})
}

async fn write_response(response: Response) {
    println!("{:?}", response);
}

trait MyService<R> {
    type Response;
    type Error;
    type Future: Future<Output = core::result::Result<Self::Response, Self::Error>>;

    fn call(&mut self, request: R) -> Self::Future;
}

#[derive(Clone, Copy)]
struct RequestHandler {}

impl MyService<Request> for RequestHandler {
    type Response = Response;
    type Error = anyhow::Error;
    type Future =
        Pin<Box<dyn Future<Output = core::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, request: Request) -> Self::Future {
        Box::pin(async move {
            println!("Ok");
            Ok(Response::ok())
        })
    }
}

#[derive(Clone, Copy)]
struct TimeoutHandler<T> {
    inner: T,
    duration: Duration,
}

impl<T> MyService<Request> for TimeoutHandler<T>
where
    T: MyService<Request> + 'static + Clone + Send,
    T::Error: Display,
    T::Future: Send,
{
    type Response = T::Response;
    type Error = anyhow::Error;
    type Future =
        Pin<Box<dyn Future<Output = core::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, request: Request) -> Self::Future {
        let mut this = self.clone();

        Box::pin(async move {
            let result = tokio::time::timeout(this.duration, this.inner.call(request)).await;
            println!("TimeoutHandler");
            match result {
                Ok(Ok(response)) => Ok(response),
                Ok(Err(e)) => Err(anyhow::Error::msg(format!("{}", e))),
                Err(_) => Err(anyhow::Error::msg("timeout")),
            }
        })
    }
}

#[derive(Clone, Copy)]
struct JsonContentType<T> {
    inner: T,
}

impl<T> MyService<Request> for JsonContentType<T>
where
    T: MyService<Request> + 'static + Clone + Send,
    T::Error: Display,
    T::Response: SetHeader,
    T::Future: Send,
{
    type Response = T::Response;
    type Error = anyhow::Error;
    type Future =
        Pin<Box<dyn Future<Output = core::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, request: Request) -> Self::Future {
        let mut this = self.clone();

        Box::pin(async move {
            match this.inner.call(request).await {
                Ok(resp) => {
                    resp.set_header("Content-Type", "application/json");
                    println!("JsonContentType");
                    Ok(resp)
                }
                Err(e) => Err(anyhow::Error::msg(format!("{}", e))),
            }
        })
    }
}

impl Server {
    fn new() -> Self {
        Server {}
    }

    async fn run_service<T>(self, mut handler: T) -> Result<()>
    where
        T: MyService<Request, Response = Response> + Send + 'static,
        T::Error: Display + Send,
        T::Future: Send,
    {
        let request = read_request().await?;
        let _ = tokio::spawn(async move {
            match handler.call(request).await {
                Ok(response) => write_response(response).await,
                Err(e) => println!("error {}", e),
            }
        })
        .await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tower_service() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let server = Server::new();
                let req_handler = RequestHandler {};
                let timeout_handler = TimeoutHandler {
                    inner: req_handler,
                    duration: Duration::from_secs(30),
                };
                let final_handler = JsonContentType {
                    inner: timeout_handler,
                };
                let _result = server.run_service(final_handler).await;
            });
    }
}
