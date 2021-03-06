#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate rand;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use chrono::{ NaiveDateTime, Local };
use rand::{ distributions, Rng };

use std::env;
use self::models::{ ReceiveApi, NewReceiveApi, ErrorTable, NewErrorTable };

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database Url must be set.");
    MysqlConnection::establish(&database_url).expect("Error : database is not connected.")
}

pub fn create_connection(conn: &MysqlConnection, token: String, ip: String) -> ReceiveApi {
    use schema::receive_api;
    let local = Local::now().naive_local();
    let keys = rand::thread_rng()
        .sample_iter(&distributions::Alphanumeric)
        .take(20)
        .collect::<String>();

    let new_schema = NewReceiveApi {
        user: keys,
        token: token,
        ip: ip,
        date: NaiveDateTime::from(local),
    };

    diesel::insert_into(receive_api::table)
        .values(&new_schema)
        .execute(conn)
        .expect("Error saving new session");

    receive_api::table.order(receive_api::token.desc())
        .first(conn)
        .unwrap()
}

pub fn error_handling(conn: &MysqlConnection, data: String) -> ErrorTable {
    use schema::errors;
    let local = Local::now().naive_local();
    let user = rand::thread_rng()
        .sample_iter(&distributions::Alphanumeric)
        .take(20)
        .collect::<String>();

    let new_schema = NewErrorTable {
        user: user,
        error: data,
        date: NaiveDateTime::from(local),
    };

    diesel::insert_into(errors::table)
        .values(&new_schema)
        .execute(conn)
        .expect("Error saving new session");

    errors::table.order(errors::user.desc())
        .first(conn)
        .unwrap()
}