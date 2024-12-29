#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper_util::rt::{TokioIo, TokioTimer};
use anyhow::Result;
use hyper::body::Bytes;
use hyper::{Request, Response};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use http_body_util::Full;

async fn hello(_: Request<impl hyper::body::Body>) -> std::result::Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello Hyper!"))))
}

async fn run_server() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_hyper_http_server(){
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(run_server()).expect("TODO: panic message");
    }
}