use serde::{Serialize};
use serde_json::json;

#[derive(Serialize)]
pub enum ResponseStatus {
    SUCCESS,
    ERROR,
    CONFLICT,
}

#[derive(Serialize)]
pub struct ApiResponse<'a, T> {
    pub status: ResponseStatus,
    pub message: &'a str,
    pub content: T,
}

impl<'a, T: Serialize> ApiResponse<'a, T> {
    pub fn success(content: T) -> serde_json::Value {
        json!(ApiResponse {
            status: ResponseStatus::SUCCESS,
            message: "",
            content
        })
    }

    pub fn error(message: &str) -> serde_json::Value {
        json!(ApiResponse {
            status: ResponseStatus::ERROR,
            message: message,
            content: "",
        })
    }

    pub fn conflict(message: &str) -> serde_json::Value {
        json!(ApiResponse {
            status: ResponseStatus::ERROR,
            message: message,
            content: "",
        })
    }
}