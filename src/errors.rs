use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AppError {
  ActixError(String),
  NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
  error_msg: String,
}
