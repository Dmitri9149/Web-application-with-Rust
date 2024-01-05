use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum CustomError {
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    error_message: String,
}

impl std::error::Error for CustomError {}

impl CustomError {
  fn error_response(&self) -> String {
    match self {
      CustomError::TeraError(msg) => {
        println!("Error in rendering the template {:?}", msg);
        msg.into()
      }
      CustomError::NotFound(msg) => {
        println!("Not found error occurred: {:?}", msg);
        msg.into()
      }
    }
  }
}

impl error::ResponseError for CustomError {
  fn status_code(&self) -> StatusCode {
    match self {

      CustomError::TeraError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
      | CustomError::NotFound(_msg) => StatusCode::NOT_FOUND,
    }
  }
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).json(
      AppErrorResponse {
        error_message: self.error_response(),
      }
    )
  }
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
      write!(f, "{}", self)
  }
}
