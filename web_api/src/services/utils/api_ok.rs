use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct APIOk {
    pub message: String,
    pub status_code: StatusCode,
    pub data: Option<serde_json::Value>,
}

impl IntoResponse for APIOk {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "StatusCode": self.status_code.as_u16(),"Message": self.message, "Data": self.data  })),
        )
            .into_response()
    }
}
