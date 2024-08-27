use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;
use utoipa::{IntoParams, OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    app_state::{AppState, Database, DatabaseDriver},
    basic::{error::Error, extract::Json, Result},
};

#[derive(OpenApi)]
#[openapi(paths(authorize, token), components(schemas(AuthorizeReq, TokenReq)))]
pub(super) struct Api;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ouath2/authorize", get(authorize))
        .route("/ouath2/token", post(token))
}

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
struct AuthorizeReq {
    response_type: String,
    state: String,
    scope: String,
    redirect_uri: String,
    client_id: String,
    user_id: String,
    user_password: String,
}

#[derive(Serialize, Debug, ToSchema)]
struct AuthorizeRes {
    state: String,
    code: String,
}

#[utoipa::path(get, path = "/ouath2/authorize", params(AuthorizeReq))]
async fn authorize(
    State(s): State<AppState>,
    Query(req): Query<AuthorizeReq>,
) -> Result<Json<AuthorizeRes>> {
    let service = OAuth2Service::new(OAuth2Repository::new(s.database.clone()));
    let res = service.authorize(&req).await?;
    Ok(Json(res))
}

#[derive(Deserialize, Debug, ToSchema)]
struct TokenReq {
    grant_type: String,
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

#[derive(Serialize, Debug, ToSchema)]
struct TokenRes {
    access_token: String,
    refresh_token: String,
    expire_in: u32,
    scope: String,
}

#[utoipa::path(
    post,
    path="/ouath2/token",
    request_body(content_type="application/x-www-form-urlencoded", 
                 content=TokenReq)
)]
async fn token(State(s): State<AppState>, Form(req): Form<TokenReq>) -> Result<Json<TokenRes>> {
    let service = OAuth2Service::new(OAuth2Repository::new(s.database.clone()));
    let res = service.token(&req).await?;
    Ok(Json(res))
}

struct OAuth2Service {
    repository: OAuth2Repository,
}

impl OAuth2Service {
    pub fn new(repository: OAuth2Repository) -> Self {
        OAuth2Service { repository }
    }

    pub async fn authorize(&self, req: &AuthorizeReq) -> Result<AuthorizeRes> {
        if req.response_type != "code" {
            return Err(Error::AppError(
                StatusCode::BAD_REQUEST,
                "invalid response_type".into(),
            ));
        }
        let cred = self.repository.find_credential(&req.client_id).await?;
        let user = self.repository.find_user(&req.user_id).await?;
        if cred.is_none() || user.is_none() {
            return Err(Error::AppError(
                StatusCode::UNAUTHORIZED,
                "invalid invalid_auth_info".into(),
            ));
        }
        let user = user.unwrap();
        if user.password != req.user_password {
            return Err(Error::AppError(
                StatusCode::UNAUTHORIZED,
                "invalid passwrod".into(),
            ));
        }
        let cred = cred.unwrap();
        let code = Uuid::new_v4().to_string();
        self.repository
            .save_authorization_code(&code, user.id, cred.id)
            .await?;
        Ok(AuthorizeRes {
            state: req.state.clone(),
            code,
        })
    }

    pub async fn token(&self, req: &TokenReq) -> Result<TokenRes> {
        if req.grant_type == "authorization_code" || req.grant_type == "refresh_token" {
            let token_type = if req.grant_type == "authorization_code" {
                0
            } else {
                2
            };
            tracing::info!(req=?req);
            let result = self.repository.find_token(&req.code, token_type).await?;
            if result.is_none() {
                return Err(Error::AppError(
                    StatusCode::BAD_REQUEST,
                    "invalid code".into(),
                ));
            }
            let token = result.unwrap();

            let access_token = Token {
                token: Uuid::new_v4().to_string(),
                token_type: 1,
                user_id: token.user_id,
                credential_id: token.credential_id,
                is_active: true,
                expired_at: chrono::Utc::now() + Duration::days(1),
            };

            let refres_token = Token {
                token: Uuid::new_v4().to_string(),
                token_type: 1,
                user_id: token.user_id,
                credential_id: token.credential_id,
                is_active: true,
                expired_at: chrono::Utc::now() + Duration::days(30),
            };
            self.repository
                .save_tokens(vec![access_token.clone(), refres_token.clone()])
                .await?;
            return Ok(TokenRes {
                access_token: access_token.token,
                refresh_token: refres_token.token,
                expire_in: 65535,
                scope: "scope".into(),
            });
        }
        return Err(Error::AppError(
            StatusCode::NOT_IMPLEMENTED,
            "not implemented".into(),
        ));
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Credential {
    pub id: i64,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub account: String,
    pub password: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Token {
    pub token: String,
    pub token_type: i32,
    pub is_active: bool,
    pub expired_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i64,
    pub credential_id: i64,
}

#[derive(Debug)]
pub struct TokenWithUser {
    pub token: String,
    pub token_type: i32,
    pub is_active: bool,
    pub expired_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i64,
    pub credential_id: i64,
    pub user: User,
}

#[derive(Clone, Debug)]
pub struct OAuth2Repository {
    db: Database,
}

impl OAuth2Repository {
    pub fn new(db: Database) -> Self {
        OAuth2Repository { db }
    }

    pub async fn find_credential(&self, client_id: &str) -> Result<Option<Credential>> {
        let result =
            sqlx::query_as::<_, Credential>(r#"select * from credential where client_id = ($1) "#)
                .bind(client_id)
                .fetch_one(&self.db.replicas_pool())
                .await;
        match result {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(e.into()),
            },
        }
    }

    pub async fn find_user(&self, account: &str) -> Result<Option<User>> {
        let result = sqlx::query_as::<_, User>(r#"select * from user where account = ($1) "#)
            .bind(account)
            .fetch_one(&self.db.replicas_pool())
            .await;
        match result {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(e.into()),
            },
        }
    }

    pub async fn save_authorization_code(
        &self,
        code: &str,
        user_id: i64,
        credential_id: i64,
    ) -> Result<Token> {
        Ok(sqlx::query_as::<_, Token>(
            r#" 
            insert into 
            token(token, token_type, is_active, user_id, credential_id, expired_at)
            values ($1, $2, $3, $4, $5, $6) returning * "#,
        )
        .bind(code)
        .bind(0)
        .bind(true)
        .bind(user_id)
        .bind(credential_id)
        .bind(chrono::Utc::now() + chrono::Duration::seconds(36400))
        .fetch_one(&self.db.sources_pool())
        .await?)
    }

    pub async fn find_token(&self, token: &str, token_type: i32) -> Result<Option<Token>> {
        let result = sqlx::query_as::<_, Token>(
            r#"
            select t.*
            from user as u
            inner join token as t on u.id = t.user_id
            where t.token = ($1) and t.token_type = ($2) "#,
        )
        .bind(token)
        .bind(token_type)
        .fetch_one(&self.db.replicas_pool())
        .await;
        match result {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(e.into()),
            },
        }
    }

    pub async fn find_user_by_token(&self, token: &str, token_type: i32) -> Result<Option<User>> {
        let result = sqlx::query_as::<_, User>(
            r#"
            select u.* 
            from user as u
            inner join token as t on u.id = t.user_id
            where t.token = ($1) and t.token_type = {$2} "#,
        )
        .bind(token)
        .bind(token_type)
        .fetch_one(&self.db.replicas_pool())
        .await;
        match result {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(e.into()),
            },
        }
    }

    pub async fn save_tokens(&self, tokens: Vec<Token>) -> Result<()> {
        let mut query_builder: QueryBuilder<DatabaseDriver> = QueryBuilder::new(
            "insert into token(token, token_type, is_active, user_id, credential_id, expired_at) ",
        );
        query_builder.push_values(tokens, |mut b, token| {
            b.push_bind(token.token)
                .push_bind(token.token_type)
                .push_bind(token.is_active)
                .push_bind(token.user_id)
                .push_bind(token.credential_id)
                .push_bind(token.expired_at);
        });
        query_builder
            .build()
            .execute(&self.db.sources_pool())
            .await?;
        Ok(())
    }
}
