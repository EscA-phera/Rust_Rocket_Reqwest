extern crate chrono;

use crate::schema::receive_api;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct ReceiveApi {
    pub token: String,
    pub ip: String,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "receive_api"]
pub struct NewReceiveApi {
    pub token: String,
    pub ip: String,
    pub date: NaiveDateTime,
}