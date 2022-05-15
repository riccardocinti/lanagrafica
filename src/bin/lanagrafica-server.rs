use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../errors.rs"]
mod errors;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../security/mod.rs"]
mod security;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();

  let shared_data = web::Data::new(AppState {
    audience: env::var("AUTH0_AUDIENCE").unwrap(),
    domain: env::var("AUTH0_DOMAIN").unwrap(),
    health_check_response: "I'm good. You've already asked me ".to_string(),
    visit_count: Mutex::new(0),
    asp_associates: Mutex::new(HashMap::new()),
  });

  let app = move || {
    App::new()
      .app_data(shared_data.clone())
      .configure(general_routes)
      .configure(aspirant_associate_routes)
  };

  let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("PORT must be a number");

  HttpServer::new(app).bind(("0.0.0.0", port))?.run().await
}
