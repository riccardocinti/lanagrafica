use crate::errors::AppError;
use crate::models::aspassociate::AspAssociate;
use crate::security::claims::Claims;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

pub async fn new_asp_associate(
  app_state: web::Data<AppState>,
  asp_associate_json: web::Json<AspAssociate>,
  _claims: Claims,
) -> Result<HttpResponse, AppError> {
  log::info!("Received a new asp associate {:?}", asp_associate_json);

  let asp_associate = AspAssociate {
    name: asp_associate_json.name.clone(),
    surname: asp_associate_json.surname.clone(),
    insert_date: Some(Utc::now().naive_utc()),
  };

  let asp_associate_store_id = Uuid::new_v4();

  app_state
    .asp_associates
    .lock()
    .unwrap()
    .insert(asp_associate_store_id.to_string(), asp_associate);

  Ok(HttpResponse::Ok().json(format!(
    "Added new aspirant associate {}",
    asp_associate_store_id
  )))
}

pub async fn get_all_asp_associates(
  app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
  let all_asp_associates: Vec<AspAssociate> = app_state
    .asp_associates
    .lock()
    .unwrap()
    .clone()
    .into_values()
    .collect();
  match all_asp_associates.len() {
    0 => Err(AppError::NotFound("Aspirant associates not found".into())),
    _ => Ok(HttpResponse::Ok().json(all_asp_associates)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http::StatusCode;
  use chrono::Utc;
  use std::collections::HashMap;
  use std::collections::HashSet;
  use std::sync::Mutex;

  #[actix_rt::test]
  async fn new_asp_associate_test() {
    let app_state: web::Data<AppState> = build_test_app_state();

    let asp_associate = web::Json(AspAssociate {
      name: "Gerry".to_string(),
      surname: "Polemica".to_string(),
      insert_date: Some(Utc::now().naive_utc()),
    });

    let resp = new_asp_associate(
      app_state,
      asp_associate,
      Claims {
        _permissions: Some(HashSet::new()),
      },
    )
    .await
    .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn get_all_asp_associates_test() {
    let asp_associate = AspAssociate {
      name: "Gerry".to_string(),
      surname: "Polemica".to_string(),
      insert_date: Some(Utc::now().naive_utc()),
    };

    let app_state: web::Data<AppState> = build_test_app_state();
    app_state
      .asp_associates
      .lock()
      .unwrap()
      .insert(Uuid::new_v4().to_string(), asp_associate);

    let resp = get_all_asp_associates(app_state).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  #[should_panic(expected = "Aspirant associates not found")]
  async fn get_all_asp_associates_not_found_test() {
    let app_state: web::Data<AppState> = build_test_app_state();

    get_all_asp_associates(app_state).await.unwrap();
  }

  fn build_test_app_state() -> web::Data<AppState> {
    web::Data::new(AppState {
      audience: "".to_string(),
      domain: "".to_string(),
      health_check_response: "".to_string(),
      asp_associates: Mutex::new(HashMap::new()),
    })
  }
}
