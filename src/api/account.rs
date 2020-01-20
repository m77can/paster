use crate::config::db::Pool;
use crate::constants;
use crate::model::response::ResponseBody;
use crate::model::user::LoginDTO;
use actix_web::{web, HttpResponse, Result};

pub async fn login(login_dto: web::Json<LoginDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match crate::service::account_service::login(login_dto.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(token_res))),
        Err(err) => Ok(err.response()),
    }
}
