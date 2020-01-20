use crate::model::user::{LoginDTO, User};
use crate::utils::token_utils;
use chrono::prelude::*;
use jsonwebtoken::Header;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
}

impl AccessToken {
    pub fn generate_token(login: User) -> String {
        let now = Local::now().timestamp();
        let payload = AccessToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.username,
        };
        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            token_utils::get_secret().as_bytes(),
        )
        .unwrap()
    }
}
