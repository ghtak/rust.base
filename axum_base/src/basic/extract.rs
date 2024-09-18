use crate::basic::error::AppReport;
use axum::{
    extract::{FromRequest, FromRequestParts},
    response::IntoResponse,
};

#[derive(Debug, FromRequest)]
#[from_request(via(axum::Json), rejection(AppReport))]
pub struct Json<T>(pub T);

#[derive(Debug, FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(AppReport))]
pub struct Path<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

// #[async_trait]
// impl<S, T> FromRequest<S> for Json<T>
// where
//     axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
//     S: Send + Sync,
// {
//     type Rejection = Error;

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let (mut parts, body) = req.into_parts();
//         let path = parts
//             .extract::<MatchedPath>()
//             .await
//             .map(|path| path.as_str().to_string())
//             .ok();
//         let req = Request::from_parts(parts, body);
//         match axum::Json::<T>::from_request(req, state).await {
//             Ok(json) => Ok(Self(json.0)),
//             Err(rejection) => Err(Error::Message(format!(
//                 "{:?} {}",
//                 path,
//                 rejection.body_text()
//             ))),
//         }
//     }
// }
