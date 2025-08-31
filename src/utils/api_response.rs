use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde::{Serialize};
use serde_json::json;

#[derive(Serialize)]
pub enum ResponseStatus {
    SUCCESS,
    ERROR,
    CONFLICT,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: ResponseStatus,
    pub message: String,
    pub content: Option<serde_json::Value>,
}

impl Responder for ApiResponse {
    type Body = actix_web::body::BoxBody;
    
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let status_code = match self.status {
            ResponseStatus::SUCCESS => StatusCode::OK,
            ResponseStatus::ERROR => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseStatus::CONFLICT => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        };

        HttpResponse::build(status_code).json(self)
    }
}

impl ApiResponse {
    pub fn success(content: serde_json::Value) -> ApiResponse {
        ApiResponse {
            status: ResponseStatus::SUCCESS,
            message: "".to_string(),
            content: Some(content)
        }
    }

    pub fn error(message: &str) -> ApiResponse {
        ApiResponse {
            status: ResponseStatus::ERROR,
            message: message.to_string(),
            content: None,
        }
    }

    pub fn conflict(message: &str) -> ApiResponse {
        ApiResponse {
            status: ResponseStatus::ERROR,
            message: message.to_string(),
            content: None,
        }
    }
}