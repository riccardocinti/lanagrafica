use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AppError {
  ActixError(String),
  NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
  error_msg: String,
}

impl AppError {
  fn error_response(&self) -> String {
    match self {
      AppError::ActixError(msg) => {
        println!("Server error occurred: {:?}", msg);
        "Internal server error".into()
      }
      AppError::NotFound(msg) => {
        println!("Not found error occurred: {:?}", msg);
        msg.into()
      }
    }
  }
}

impl error::ResponseError for AppError {
  fn status_code(&self) -> StatusCode {
    match self {
      AppError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::NotFound(_msg) => StatusCode::NOT_FOUND,
    }
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).json(AppErrorResponse {
      error_msg: self.error_response(),
    })
  }
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{}", self)
  }
}

impl From<actix_web::error::Error> for AppError {
  fn from(err: actix_web::error::Error) -> Self {
    AppError::ActixError(err.to_string())
  }
}
