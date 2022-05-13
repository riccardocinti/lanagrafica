use actix_web::web;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AspAssociate {
  pub name: String,
  pub surname: String,
  pub insert_date: Option<NaiveDateTime>,
}

impl From<web::Json<AspAssociate>> for AspAssociate {
  fn from(asp_associate: web::Json<AspAssociate>) -> Self {
    AspAssociate {
      name: asp_associate.name.clone(),
      surname: asp_associate.surname.clone(),
      insert_date: None,
    }
  }
}
