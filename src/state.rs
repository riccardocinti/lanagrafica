use crate::models::aspassociate::AspAssociate;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AppState {
  pub audience: String,
  pub domain: String,
  pub health_check_response: String,
  pub asp_associates: Mutex<HashMap<String, AspAssociate>>,
}
