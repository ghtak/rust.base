use std::future::Future;
use anyhow::Error;
use tokio::net::{TcpListener, TcpStream};

struct Server {
    address: String,
}

struct Request {}

struct Response {}

async fn read_request(stream: &mut TcpStream) -> Result<Request, Error> {
    Ok(Request {})
}

async fn write_response(stream: TcpStream, response: Response) {}

impl Server {
    fn new(address: &str) -> Self {
        Server {
            address: address.into()
        }
    }

    async fn run<F, Fut>(self, handler: F) -> Result<(), Error>
    where
        F: Fn(Request) -> Fut + Send + Copy + 'static,
        Fut: Future<Output=Response> + Send,
    {
        let listener = TcpListener::bind(&self.address).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;

            let request = read_request(&mut stream).await?;

            tokio::spawn(async move {
                let response = handler(request).await;
                write_response(stream, response).await;
            });
        }
    }
}

async fn handler_impl(request: Request) -> Response {
    Response{}
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
                let server = Server::new("127.0.0.1:5000");
                server.run(handler_impl).await?;
                Ok(())
            }).expect("TODO: panic message");
    }
}