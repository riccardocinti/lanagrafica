use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg
  .route("/", web::get().to(health_check_handler))
  .route("/health", web::get().to(health_check_handler));
}

