use crate::schema::user::{self, dsl::*};
use crate::utils::hasher::{HASHER, PWD_SCHEME_VERSION};
use bcrypt::{hash, verify};
use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::config::db::Connection;

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "user"]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "user"]
pub struct UserChanged {
    pub username: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn login(login: LoginDTO, conn: &Connection) -> Option<User> {
        let persisted_user = user
            .filter(username.eq(&login.username))
            .get_result::<User>(conn);
        //        info!("{:?}", persisted_user);
        info!("Start");
        match persisted_user {
            Ok(user_to_verify) => {
                info!("Start");
                match verify(
                    login.password + &user_to_verify.salt,
                    &user_to_verify.password_hash,
                ) {
                    Ok(result) => {
                        if result {
                            info!("User {} login successfully", user_to_verify.username);
                            info!("end");

                            Some(user_to_verify)
                        } else {
                            info!("User {} password is error", user_to_verify.username);
                            None
                        }
                    }
                    Err(e) => {
                        info!("Can not Found user {}, the error is {}", login.username, e);
                        None
                    }
                }
            }

            Err(e) => {
                info!("Can not Found user {}, the error is {}", login.username, e);
                None
            }
        }
    }
}

#[test]
fn test() {
    let password = String::from("qwer1234asdfghiloveyou");
    println!("{:?}", hash(password, 4));
}
