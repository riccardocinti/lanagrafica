use actix_web::web;

pub fn general_routes(cfg: &mut web::ServerConfig) {
  cfg.route("/health", web::get().to(health_check_handler));
}

