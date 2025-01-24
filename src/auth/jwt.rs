use crate::models::auth::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn create_jwt(user_id: &Uuid, secret_key: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration =
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + Duration::from_secs(60 * 60);

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration.as_secs() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
}

pub fn verify_jwt(token: &str, secret_key: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}
