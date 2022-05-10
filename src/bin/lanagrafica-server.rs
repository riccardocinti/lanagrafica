use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
#[path = "../errors.rs"]
mod errors;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  let shared_data = web::Data::new(AppState {
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
