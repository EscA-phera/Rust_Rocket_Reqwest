#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use chrono::{ NaiveDateTime, Local };

use std::env;

use self::models::{ ReceiveApi, NewReceiveApi };

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database Url must be set.");
    MysqlConnection::establish(&database_url).expect(&format!("Error : {}", database_url))
}

pub fn create_connection(conn: &MysqlConnection, token: String, ip: String) -> ReceiveApi {
    use schema::receive_api;
    let local = Local::now().naive_local();

    let new_schema = NewReceiveApi {
        token: token,
        ip: ip,
        date: NaiveDateTime::from(local)
    };

    diesel::insert_into(receive_api::table)
        .values(&new_schema)
        .execute(conn)
        .expect("Error saving new session");

    receive_api::table.order(receive_api::token.desc())
        .first(conn)
        .unwrap()
}