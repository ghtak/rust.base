use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use utoipa::{IntoParams, OpenApi, ToSchema};
use uuid::Uuid;

use crate::basic::{error::Error, extract::Json, state::BasicState, Result};

#[derive(OpenApi)]
#[openapi(paths(authorize, token), components(schemas(AuthorizeReq, TokenReq)))]
pub(super) struct Api;

pub fn router() -> Router<BasicState> {
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
    State(s): State<BasicState>,
    Query(req): Query<AuthorizeReq>,
) -> Result<Json<AuthorizeRes>> {
    let service = OAuth2Service::new(s.oauth2_state.repository.clone());
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
async fn token(State(s): State<BasicState>, Form(req): Form<TokenReq>) -> Result<Json<TokenRes>> {
    let service = OAuth2Service::new(s.oauth2_state.repository.clone());
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
                "400".into(),
                "invalid response_type".into(),
            ));
        }
        if (req.client_id != "sample_client_id")
            || (req.user_id != "user_id")
            || (req.user_password != "passwd")
        {
            return Err(Error::AppError(
                StatusCode::UNAUTHORIZED,
                "400".into(),
                "invalid invalid_auth_info".into(),
            ));
        }

        let code = Uuid::new_v4().to_string();

        self.repository.save_by_code(&code, &req.user_id).await?;

        Ok(AuthorizeRes {
            state: req.state.clone(),
            code,
        })
    }

    pub async fn token(&self, req: &TokenReq) -> Result<TokenRes> {
        if req.grant_type == "authorization_code" || req.grant_type == "refresh_token" {
            let result = self.repository.find_by_code(&req.code).await;
            if result.is_none() {
                return Err(Error::AppError(
                    StatusCode::BAD_REQUEST,
                    "400".into(),
                    "invalid code".into(),
                ));
            }
            let user_id = result.unwrap();
            let access_token = Uuid::new_v4().to_string();
            let refresh_token = Uuid::new_v4().to_string();
            self.repository
                .save_by_code(&access_token, &user_id)
                .await?;
            self.repository
                .save_by_code(&refresh_token, &user_id)
                .await?;
            return Ok(TokenRes {
                access_token,
                refresh_token,
                expire_in: 65535,
                scope: "scope".into(),
            });
        }
        return Err(Error::AppError(
            StatusCode::NOT_IMPLEMENTED,
            "501".into(),
            "not implemented".into(),
        ));
    }
}

#[derive(Clone, Debug)]
pub struct OAuth2Repository {
    map: Arc<RwLock<HashMap<String, String>>>,
}

impl OAuth2Repository {
    pub fn new() -> Self {
        OAuth2Repository {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn save_by_code<'a>(&self, code: &'a str, user_id: &'a str) -> Result<()> {
        let mut map = self.map.write().await;
        map.insert(code.into(), user_id.into());
        Ok(())
    }

    pub async fn find_by_code<'a>(&self, code: &'a str) -> Option<String> {
        let map = self.map.read().await;
        if map.contains_key(code) {
            Some(map.get(code).unwrap().into())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct OAuth2State {
    repository: OAuth2Repository,
}

impl OAuth2State {
    pub fn new(repository: OAuth2Repository) -> Self {
        OAuth2State { repository }
    }
}
