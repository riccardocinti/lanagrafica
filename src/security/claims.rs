use crate::errors::AppError;
use crate::state::AppState;
use actix_web::{client::Client, http::Uri, web::Data, Error, FromRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{
  decode, decode_header,
  jwk::{AlgorithmParameters, JwkSet},
  Algorithm, DecodingKey, Validation,
};
use serde::Deserialize;
use std::{collections::HashSet, future::Future, pin::Pin};

#[derive(Debug, Deserialize)]
pub struct Claims {
  pub _permissions: Option<HashSet<String>>,
}

impl FromRequest for Claims {
  type Error = Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  type Config = ();

  fn from_request(
    req: &actix_web::HttpRequest,
    _payload: &mut actix_web::dev::Payload,
  ) -> Self::Future {
    let config = req.app_data::<Data<AppState>>().unwrap().clone();
    let extractor = BearerAuth::extract(req);
    Box::pin(async move {
      let credentials = extractor.await.map_err(AppError::Authentication)?;
      let token = credentials.token();
      let header = decode_header(token).map_err(AppError::Decode)?;
      let kid = header.kid.unwrap();
      let domain = config.domain.as_str();
      let jwks: JwkSet = Client::new()
        .get(
          Uri::builder()
            .scheme("https")
            .authority(domain)
            .path_and_query("/.well-known/jwks.json")
            .build()
            .unwrap(),
        )
        .send()
        .await?
        .json()
        .await?;
      let jwk = jwks.find(&kid).unwrap();
      match jwk.clone().algorithm {
        AlgorithmParameters::RSA(ref rsa) => {
          let mut validation = Validation::new(Algorithm::RS256);
          validation.set_audience(&[config.audience.clone()]);
          validation.set_issuer(&[Uri::builder()
            .scheme("https")
            .authority(domain)
            .path_and_query("/")
            .build()
            .unwrap()]);
          let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).map_err(AppError::Decode)?;
          let token = decode::<Claims>(token, &key, &validation).map_err(AppError::Decode)?;
          Ok(token.claims)
        }
        _algorithm => Err(AppError::ActixError("Unsupported algorithm error".to_string()).into()),
      }
    })
  }
}
