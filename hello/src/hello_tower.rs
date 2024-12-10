use std::future::Future;

struct Server {}

struct Request {}

#[derive(Debug)]
struct Response {
    code: usize
}

impl Response{
    fn ok() -> Self{
        Response{code:200}
    }
    fn not_found() -> Self {
        Response{code:404}
    }
}


type Result<T> = core::result::Result<T, anyhow::Error>;

async fn read_request() -> Result<Request> {
    Ok(Request {})
}

async fn write_response(response: Response) {
    println!("{:?}", response);
}

impl Server {
    fn new() -> Self {
        Server {}
    }

    async fn run<F, Fut>(self, handler: F) -> Result<()>
    where
        F: Fn(Request) -> Fut + Send + Copy + 'static,
        Fut: Future<Output = Result<Response>> + Send,
    {
        let request = read_request().await?;
        tokio::spawn(async move {
            match handler(request).await {
                Ok(response) => write_response(response).await,
                Err(e) => println!("error {:?}", e),
            }
        });
        Ok(())
    }
}

async fn handle_request(request: Request) -> Result<Response> {
    Ok(Response::ok())
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
                let _result = server.run(handle_request).await;
            });
    }
}
