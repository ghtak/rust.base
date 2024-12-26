#![allow(dead_code)]
#![allow(unused_variables)]
use anyhow::Ok;
#[warn(unused_imports)]
use anyhow::Result;
use http_body_util::Full;
use hyper::body;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Bytes, Request, Response};
use hyper_util::rt::{TokioIo, TokioTimer};
use hyper_util::service::TowerToHyperService;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower::{Service, ServiceExt};
use tower_http::trace::{Trace, TraceLayer};

async fn handler(request: Request<body::Incoming>) -> Result<Response<Full<Bytes>>> {
    Ok(Response::new(Full::new(Bytes::from("Hello Hyper!"))))
}

async fn run_server() -> Result<()>
{
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            let service = ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .service_fn(handler);
            let service = TowerToHyperService::new(service);
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service)
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
    fn test_tower_http() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                run_server().await;
            });
    }
}
