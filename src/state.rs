use crate::models::aspassociate::AspAssociate;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AppState {
  pub health_check_response: String,
  pub visit_count: Mutex<u32>,
  pub asp_associates: Mutex<HashMap<String, AspAssociate>>,
}
