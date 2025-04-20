use crate::database::DatabasePool;
use crate::redis::RedisPool;
use crate::{app_context::AppContext, extract_ext::Json};
use axum::routing::get;
use axum::Router;
use axum::{extract::State, response::IntoResponse};
use deadpool_redis::redis::cmd;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name="Database", description="Database API"),
    ),
    paths(database_post, database_get)
)]
pub struct DatabaseApiDoc;

pub fn database_router<S>(state: AppContext) -> Router<S> {
    Router::new()
        .route("/database", get(database_get).post(database_post))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct User {
    name: String,
    mail: String,
}

#[utoipa::path(post, path="/database", request_body=User)]
async fn database_post(
    State(pool): State<DatabasePool>,
    State(redis_pool): State<RedisPool>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let result = sqlx::query("insert into users(name, email) values($1, $2)")
        .bind(user.name.clone())
        .bind(user.mail.clone())
        .execute(&pool)
        .await;
    print!("{:?}", result);
    let mut conn = redis_pool.get().await.unwrap();
    cmd("SET")
        .arg(user.name.as_str())
        .arg(user.mail.as_str())
        .query_async::<()>(&mut conn)
        .await
        .unwrap();
    return "get";
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
struct UserModel {
    id: u32,
    name: String,
    email: String,
}

#[utoipa::path(get, path = "/database")]
async fn database_get(State(pool): State<DatabasePool>) -> impl IntoResponse {
    let users: Vec<UserModel> = sqlx::query_as::<_, UserModel>("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(users)
}
