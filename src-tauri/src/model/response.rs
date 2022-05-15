use std::result;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub result: String,
    pub data: Option<T>,
    pub errors: Option<Vec<ResponseError>>,
}

impl<T> ApiResponse<T> {
    pub fn result(self, tag: &str) -> Result<T> {
        match self.result.as_str() {
            "ok" => Ok(self.data.unwrap()),
            _ => Err(ServiceError::ApiError {
                errors: self
                    .errors
                    .unwrap_or_else(|| vec![ResponseError::default()]),
                tag: tag.to_owned(),
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseError {
    id: String,
    status: u32,
    title: String,
    detail: String,
}

impl Default for ResponseError {
    fn default() -> Self {
        Self {
            id: Default::default(),
            status: 500,
            title: "unknown error".to_owned(),
            detail: "something went wrong".to_owned(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("invalid arguments: {}", .0)]
    InvalidArguments(String),

    #[error("failed call to api \"{}\", errors: {:#?}", .tag, .errors)]
    ApiError {
        errors: Vec<ResponseError>,
        tag: String,
    },

    #[error("failed http requests")]
    HttpError(#[from] reqwest::Error),
}

pub type Result<T> = result::Result<T, ServiceError>;
