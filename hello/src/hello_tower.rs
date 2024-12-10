use anyhow::Error;
use std::future::Future;

struct Server {}

struct Request {}

struct Response {}

async fn read_request() -> Result<Request, Error> {
    Ok(Request {})
}

async fn write_response(response: Response) {}

impl Server {
    fn new() -> Self {
        Server {}
    }

    async fn run<F, Fut>(self, handler: F) -> Result<(), Error>
    where
        F: Fn(Request) -> Fut + Send + Copy + 'static,
        Fut: Future<Output = Response> + Send,
    {   
        let request = read_request().await?;
        tokio::spawn(async move {
            let response = handler(request).await;
            write_response(response).await;
        });
        Ok(())
    }
}

async fn handle_request(request: Request) -> Response {
    Response {}
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
