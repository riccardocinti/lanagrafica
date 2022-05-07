use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/", web::get().to(health_check_handler))
    .route("/health", web::get().to(health_check_handler));
}

pub fn aspirant_associate_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/aspirant-associate")
      .route("/", web::post().to(new_asp_associate))
      .route("/", web::get().to(get_all_asp_associates)),
      // .route("/{params", web::get().to(get_asp_associate)),
  );
}
