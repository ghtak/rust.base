use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use error_stack::Report;
use redis::cmd;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::{
    app_state::{AppState, RedisConnection},
    basic::{
        depends::Depends,
        error::{AppError, AppReport, AppResult, Error, JsonResult},
        extract::{Json, Path},
        redis::RedisRepository,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        sample_post,
        sample_path,
        get_cookie,
        set_cookie,
        get_redis,
        set_redis,
        get_redis1,
        set_redis_repo,
        get_redis_repo,
        error_stack_result,
    ),
    components(schemas(Sample))
)]
pub(super) struct Api;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sample/", post(sample_post))
        .route("/sample/:user_id/teams/:team_id", get(sample_path))
        .route("/sample/set_cookie", get(set_cookie))
        .route("/sample/get_cookie", get(get_cookie))
        .route("/sample/set_redis", get(set_redis))
        .route("/sample/get_redis", get(get_redis))
        .route("/sample/get_redis1", get(get_redis1))
        .route("/sample/set_redis_repo", get(set_redis_repo))
        .route("/sample/get_redis_repo", get(get_redis_repo))
        .route("/sample/error_stack_result", get(error_stack_result))
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
    Ok(c.value().to_string())
}

#[utoipa::path(get, path = "/sample/set_redis")]
async fn set_redis(State(s): State<AppState>) -> Result<impl IntoResponse, Error> {
    let mut conn = s
        .redis_pool
        .get()
        .await
        .map_err(|e| Error::Message(e.to_string()))?;
    cmd("SET")
        .arg("test_key")
        .arg("test_value")
        .query_async(&mut *conn)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;
    Ok("".to_string())
}

#[utoipa::path(get, path = "/sample/get_redis")]
async fn get_redis(State(s): State<AppState>) -> Result<impl IntoResponse, Error> {
    let mut conn = s
        .redis_pool
        .get()
        .await
        .map_err(|e| Error::Message(e.to_string()))?;
    let value: Option<String> = cmd("GET")
        .arg("test_key")
        .query_async(&mut *conn)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;
    if let Some(v) = value {
        return Ok(v);
    }
    Ok("None".to_string())
}

#[utoipa::path(get, path = "/sample/get_redis1")]
async fn get_redis1(Depends(mut c): Depends<RedisConnection>) -> Result<impl IntoResponse, Error> {
    let value: Option<String> = cmd("GET")
        .arg("test_key")
        .query_async(&mut *c)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;
    if let Some(v) = value {
        return Ok(v);
    }
    Ok("None".to_string())
}

#[utoipa::path(get, path = "/sample/get_redis_repo")]
async fn get_redis_repo(
    Depends(repo): Depends<RedisRepository>,
) -> Result<impl IntoResponse, AppReport> {
    let resp = if let Some(value) = repo.get_string("test_key").await? {
        value
    } else {
        "None".to_string()
    };
    Ok(resp)
}

#[utoipa::path(get, path = "/sample/set_redis_repo")]
async fn set_redis_repo(
    Depends(repo): Depends<RedisRepository>,
) -> Result<impl IntoResponse, AppReport> {
    repo.set_string("test_key", "test_value").await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    code: u32,
    message: String,
}

fn return_error() -> AppResult<Resp> {
    // Ok(Resp {
    //     code: 16,
    //     message: "OkMessage".to_string(),
    // })
    Err(Report::from(AppError::BadRequest("test".to_string())))
}

#[utoipa::path(get, path = "/sample/error_stack_result")]
async fn error_stack_result() -> JsonResult<Resp> {
    Ok(axum::Json(return_error()?))
}
