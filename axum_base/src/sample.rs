use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use redis::cmd;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::{
    app_state::AppState,
    basic::{
        error::Error,
        extract::{Json, Path},
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(sample_post, sample_path, get_cookie, set_cookie, get_redis, set_redis),
    components(schemas(Sample))
)]
pub(super) struct Api;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/sample/", post(sample_post))
        .route("/sample/:user_id/teams/:team_id", get(sample_path))
        .route("/sample/set_cookie", get(set_cookie))
        .route("/sample/get_cookie", get(get_cookie))
        .route("/sample/set_redis", get(set_redis))
        .route("/sample/get_redis", get(get_redis))
        .with_state(state)
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

#[utoipa::path(get, path = "/sample/set_redis")]
async fn set_redis(State(s): State<AppState>) -> Result<impl IntoResponse, Error> {
    if let Some(redis_p) = s.redis_pool {
        let mut conn = redis_p
            .get()
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
        cmd("SET")
            .arg("test_key")
            .arg("test_value")
            .query_async(&mut *conn)
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
    }
    Ok("".to_owned())
}

#[utoipa::path(get, path = "/sample/get_redis")]
async fn get_redis(State(s): State<AppState>) -> Result<impl IntoResponse, Error> {
    if let Some(redis_p) = s.redis_pool {
        let mut conn = redis_p
            .get()
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
        let value: Option<String> = cmd("GET")
            .arg("test_key")
            .query_async(&mut *conn)
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
        if let Some(v) = value {
            return Ok(v)
        }
    }
    Ok("None".to_owned())
}
