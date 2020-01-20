use crate::prelude::*;
use diesel::{
    mysql::MysqlConnection,
    r2d2::{self, ConnectionManager},
};

pub type Connection = MysqlConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn new_pool<S: Into<String>>(database_url: S) -> Result<Pool> {
    let manager = ConnectionManager::<Connection>::new(database_url.into());
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}
