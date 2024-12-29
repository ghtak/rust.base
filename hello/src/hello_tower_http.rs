#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use anyhow::Ok;
use anyhow::Result;
use http_body_util::Full;
use hyper::body;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Bytes, Request, Response};
use hyper_util::rt::{TokioIo, TokioTimer};
use hyper_util::service::TowerToHyperService;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower::{Service, ServiceExt};
use tower_http::trace::{Trace, TraceLayer};

async fn handler(request: Request<body::Incoming>) -> Result<Response<Full<Bytes>>> {
    Ok(Response::new(Full::new(Bytes::from("Hello Hyper!"))))
}

struct TowerAdapter<S> {
    service: S,
}

impl<S> TowerAdapter<S> {
    fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S, R> hyper::service::Service<R> for TowerAdapter<S>
where
    S: tower::Service<R> + Clone + 'static + Send,
    R: 'static + Send,
    S::Future: 'static + Send
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send >> ;

    fn call(&self, req: R) -> Self::Future {
        let mut service = self.service.clone();
        Box::pin(async move {
            let x = service.ready().await?;
            x.call(req).await
        })
    }
}

async fn run_server() -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            let service = ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .service_fn(handler);
            let adapter = TowerAdapter::new(service);
            //let service = TowerToHyperService::new(service);
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, adapter)
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
