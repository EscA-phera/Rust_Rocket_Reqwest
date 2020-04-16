extern crate chrono;

use crate::schema::{ errors, receive_api };
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct ReceiveApi {
    pub user: String,
    pub token: String,
    pub ip: String,
    pub date: NaiveDateTime,
}

#[derive(Queryable)]
pub struct ErrorTable {
    pub user: String,
    pub error: String,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "receive_api"]
pub struct NewReceiveApi {
    pub user: String,
    pub token: String,
    pub ip: String,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "errors"]
pub struct NewErrorTable {
    pub user: String,
    pub error: String,
    pub date: NaiveDateTime,
}