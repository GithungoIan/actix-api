use crate::errors::ServiceError;
use actix_web::http::StatusCode;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let url = format!("{}.well-known/jwks.json", authority);
    let jwks = fetch_jwks(&url).await.expect("failed to fetch jwks");
    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse jwk".to_string(),
            ))
        }
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let  res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    return Ok(val);
}
  
