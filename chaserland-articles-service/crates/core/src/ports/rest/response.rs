use axum::{Json, response::IntoResponse};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub struct ErrorResponse {
    pub status: StatusCode,
    pub error_messaage: String,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, error_messaage: String) -> Self {
        Self {
            status,
            error_messaage,
        }
    }

    pub fn new_default(status: StatusCode) -> Self {
        Self {
            status,
            error_messaage: status.as_str().into(),
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ErrorBody {
    pub error: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = Json(ErrorBody {
            error: self.error_messaage,
        })
        .into_response();
        *response.status_mut() = self.status;
        response
    }
}
