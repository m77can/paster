use crate::model::user::LoginDTO;
use crate::{
    config::db::Pool, constants, error::ServiceError, model::access_token::AccessToken,
    model::user::User,
};
use actix_web::{http::StatusCode, web};

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub user: String,
    pub token: String,
    pub token_type: String,
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<UserToken, ServiceError> {
    match User::login(login, &pool.get().unwrap()) {
        Some(logged_user) => {
            match serde_json::from_value(json!({ 
                "user" : logged_user.username,
                "token": AccessToken::generate_token(logged_user),
                "token_type": "bearer" }))
            {
                Ok(token_res) => Ok(token_res),
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    constants::CODE_SERVER_ERROR_INTERNAL.to_string(),
                    constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
                )),
            }
        }
        None => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::CODE_SERVER_ERROR_INTERNAL.to_string(),
            constants::MESSAGE_LOGIN_FAILED.to_string(),
        )),
    }
}
