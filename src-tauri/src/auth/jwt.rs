use anyhow::Result;
use chrono::{Duration, Utc};
use dotenv::dotenv;
use entity::user;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub user: user::Model,
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_token(user: user::Model) -> Result<String> {
    dotenv().ok();
    let secret = std::env::var("SECRET_KEY").expect("Failed to get secret");
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::days(1)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        user,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    Ok(token)
}

pub fn validate_token(token: &str) -> Result<user::Model> {
    let secret = std::env::var("SECRET_KEY").expect("Failed to get secret");

    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims.user)
}
