use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
  let health_check_response = &app_state.health_check_response;
  let mut visit_count = app_state.visit_count.lock().unwrap();
  let response = format!("{} {} times", health_check_response, visit_count);
  *visit_count += 1;
  HttpResponse::Ok().json(&response)
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http::StatusCode;
  use std::collections::HashMap;
  use std::sync::Mutex;

  #[actix_rt::test]
  async fn health_check_test() {
    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      asp_associates: Mutex::new(HashMap::new()),
    });

    let resp = health_check_handler(app_state).await;
    assert_eq!(resp.status(), StatusCode::OK);
  }
}
