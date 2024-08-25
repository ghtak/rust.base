use axum::{routing::get, Router};
use tokio::signal;
use utoipa::{
    openapi::{Info, OpenApiBuilder},
    OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
mod basic;

mod sample;

// #[derive(OpenApi)]
// #[openapi(
//     info(description = "Basic Api description"),
//     nest(
//         // you can nest sub apis here
//         (path = "/sample", api = sample::Api)
//     )
// )]
// struct ApiDoc;
#[derive(OpenApi)]
#[openapi(info(description = "Basic Api description"))]
struct BasicApiDoc;

#[tokio::main]
async fn main() {
    let mut api_doc = BasicApiDoc::openapi();
    api_doc.merge(sample::Api::openapi());
    let env = basic::env::Env::new("dev.toml");
    basic::tracing::init(&env);
    let state = basic::state::State {};
    tokio::spawn(async move {
        tracing::info!("init with env {env:?}");
        let router = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
            .route("/", get(|| async move { "Hello Axum" }))
            .merge(sample::router())
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(env.server.address.as_str())
            .await
            .expect("listen fail");
        axum::serve(listener, router).await.expect("serve fail");
    });
    signal::ctrl_c().await.expect("signal fail");
}
