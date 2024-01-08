use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiOkResponse {
    pub status: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiOkResponseBuilder {
    pub status: String,
    pub message: String,
}

impl ApiOkResponse {
    pub fn new() -> ApiOkResponseBuilder {
        ApiOkResponseBuilder {
            status: String::new(),
            message: String::new(),
        }
    }
}

impl ApiOkResponseBuilder {
    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status.to_string();
        self
    }

    pub fn message(&mut self, message: &str) -> &mut Self {
        self.message = message.to_string();
        self
    }

    pub fn build(&mut self) -> ApiOkResponse {
        ApiOkResponse {
            status: self.status.to_owned(),
            message: self.message.to_owned(),
        }
    }
}
