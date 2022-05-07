use actix_web::{web, App, HttpServer};
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  let shared_data = web::Data::new(AppState {
    health_check_response: "I'm good. You've already asked me ".to_string(),
    visit_count: Mutex::new(0),
  });

  let app = move || {
    App::new()
      .app_data(shared_data.clone())
      .configure(general_routes)
  };

  let host = env::var("HOST").expect("Host not set");
  let port = env::var("PORT").expect("Port not set");

  HttpServer::new(app)
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
