use axum::{
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::basic::{
    error::Error,
    extract::{Json, Path},
};

#[derive(OpenApi)]
#[openapi(
    paths(sample_post, sample_path, get_cookie, set_cookie),
    components(schemas(Sample))
)]
pub(super) struct Api;

pub fn router() -> Router {
    Router::new()
        .route("/sample/", post(sample_post))
        .route("/sample/:user_id/teams/:team_id", get(sample_path))
        .route("/sample/set_cookie", get(set_cookie))
        .route("/sample/get_cookie", get(get_cookie))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Sample {
    name: String,
    detail: String,
}

#[utoipa::path(
    post,
    path="/sample/",
    request_body=Sample,
)]
async fn sample_post(Json(sample): Json<Sample>) -> impl IntoResponse {
    Json(sample)
}

#[derive(Debug, Deserialize, Serialize)]
struct SampleParams {
    user_id: u32,
    team_id: u32,
}

#[utoipa::path(
    get,
    path="/sample/{user_id}/teams/{team_id}",
    params(
        ("user_id", description = "user_id"),
        ("team_id", description = "team_id"),
    )
)]
async fn sample_path(Path(sp): Path<SampleParams>) -> impl IntoResponse {
    axum::Json(sp)
}

#[utoipa::path(get, path = "/sample/set_cookie")]
async fn set_cookie(jar: CookieJar) -> impl IntoResponse {
    let set_cookie = jar.add(Cookie::new("foo", "bar"));
    (set_cookie, "SetCookie")
}

#[utoipa::path(get, path = "/sample/get_cookie")]
async fn get_cookie(jar: CookieJar) -> Result<impl IntoResponse, Error> {
    let c = jar.get("foo").ok_or(Error::Message("xx".into()))?;
    Ok(c.value().to_owned())
}
