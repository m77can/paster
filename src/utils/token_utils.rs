use crate::{config::db::Pool, model::access_token::AccessToken};
use actix_web::web;
use jsonwebtoken::{TokenData, Validation};

pub fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into())
}

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<AccessToken>> {
    jsonwebtoken::decode::<AccessToken>(&token, get_secret().as_bytes(), &Validation::default())
}

pub fn verify_token(
    token_data: &TokenData<AccessToken>,
    pool: &web::Data<Pool>,
) -> Result<String, String> {
    Ok(String::from("OK"))
}
