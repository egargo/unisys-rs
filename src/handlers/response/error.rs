use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiErrorResponseBuilder {
    pub status: String,
    pub message: String,
}

impl ApiErrorResponse {
    pub fn new() -> ApiErrorResponseBuilder {
        ApiErrorResponseBuilder {
            status: String::new(),
            message: String::new(),
        }
    }
}

impl ApiErrorResponseBuilder {
    /// Sets the status value ([`StatusCode`]) of the response.
    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status.to_string();
        self
    }

    /// Sets the message of the response.
    pub fn message(&mut self, message: &str) -> &mut Self {
        self.message = message.to_string();
        self
    }

    /// Builds the ApiErrorResponse.
    pub fn build(&mut self) -> ApiErrorResponse {
        ApiErrorResponse {
            status: self.status.to_owned(),
            message: self.message.to_owned(),
        }
    }
}
