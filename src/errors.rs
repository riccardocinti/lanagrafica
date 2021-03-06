use actix_web::{error, http::StatusCode, HttpResponse, Result};
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use serde::{Serialize, Serializer};
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AppError {
  ActixError(String),
  NotFound(String),
  #[serde(serialize_with = "use_display")]
  Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
  #[serde(serialize_with = "use_display")]
  Decode(jsonwebtoken::errors::Error),
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
  error_msg: String,
}

impl AppError {
  fn error_response(&self) -> String {
    match self {
      AppError::ActixError(msg) => {
        log::error!("Internal server error: {}", msg);
        "Internal server error".into()
      }
      AppError::NotFound(msg) => {
        log::error!("Resource not found: {}", msg);
        msg.into()
      }
      AppError::Authentication(msg) => {
        log::error!("Missing authentication error: {}", msg);
        "Requires authentication".to_string()
      }
      AppError::Decode(msg) => {
        log::error!("Authorization header error: {}", msg);
        "Authorization header value must follow this format: Bearer access-token".to_string()
      }
    }
  }
}

impl error::ResponseError for AppError {
  fn status_code(&self) -> StatusCode {
    match self {
      AppError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::NotFound(_msg) => StatusCode::NOT_FOUND,
      AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
      AppError::Decode(_) => StatusCode::UNAUTHORIZED,
    }
  }

  fn error_response(&self) -> HttpResponse {
    match self {
      _ => HttpResponse::build(self.status_code()).json(AppErrorResponse {
        error_msg: self.error_response(),
      }),
    }
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

fn use_display<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
  T: fmt::Display,
  S: Serializer,
{
  serializer.collect_str(value)
}
