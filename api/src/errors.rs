use poem::{Body, Response};
use poem::error::ResponseError;
use poem::http::StatusCode;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Internal server error")]
    InternalServerError,
    #[error("No password found")]
    NoPassword,
}

impl ResponseError for ApiError {
    fn status(&self) -> StatusCode {
        match self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NoPassword => StatusCode::NOT_FOUND,
        }
    }

    fn as_response(&self) -> Response {
        let status = self.status();
        let body = Body::from_json(json!({
            "status": status.as_u16(),
            "message": self.to_string(),
        })).unwrap();
        Response::builder().status(status).body(body)
    }
}
