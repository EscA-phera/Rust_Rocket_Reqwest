#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use pre_kan::create_connection;
use pre_kan::establish_connection;

pub mod frontend;
pub mod lib_redis;

use std::string::String;
use std::net::SocketAddr;

use rocket::request::Request;
use rocket::response::{ Response, Responder, Result };
use rocket::http::{ Status, ContentType };

use rocket_contrib::templates;
use rocket_contrib::json::{ Json, JsonValue };

use regex::Regex;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct PlaceType {
    token: String,
    ip: String,
}

#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[post("/api/v1/place", format = "application/json", data = "<data>")]
pub fn place(data: Json<PlaceType>, remote_addr: SocketAddr) -> ApiResponse {
    let check_string: u32 = String::from(&data.0.token).chars().count() as u32;
    let length: u32 = 40;
    
    let special_letter = Regex::new(r"[!@#$%^&*(),.?:{}|<>]").unwrap();
    let result: String = String::from(special_letter.replace_all(&data.0.token, "nohack"));

    let req_url: String = format!("http://{}/kcsapi/api_get_member/deck", &data.0.ip);
    let client_ip: String = format!("{}", &remote_addr);
    
    let status_point: String;
    let data_status: String;

    if String::from(&data.0.token).contains("\"") {
        status_point = String::from("failed");
        data_status = String::from("your token is something wrong.");
    }

    else if result.contains("'") {
        status_point = String::from("failed");
        data_status = String::from("your token is something wrong.");
    }

    else if result.contains("%") {
        status_point = String::from("failed");
        data_status = String::from("your token is something wrong.");
    }

    else {
        if result == data.0.token {
            if check_string == length {
                let result_data = format!("{:?}", post_data(req_url, data.0.token, client_ip));
                status_point = String::from("success!");
                data_status = String::from(result_data);
            }

            else {
                status_point = String::from("failed");
                data_status = String::from("your token is something wrong.");
            }
        }

        else {
            status_point = String::from("failed");
            data_status = String::from("your token is something wrong.");
        }
    }

        ApiResponse {
            json: json!({
                "status": &status_point,
                "data": &data_status
            }),
            status: Status::NotImplemented
        }
    }

pub fn post_data(urls: String, token: String, ip: String) -> std::result::Result<reqwest::blocking::Response, reqwest::Error> {
    let send_api = json!(
        {
            "api_token": token,
            "api_verno": 1,
        }
    );

    let req = reqwest::blocking::Client::new();

    let res = req.post(&urls)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&send_api)
        .send();

    let db_connection = establish_connection();
    create_connection(&db_connection, token, ip);

    return res;
}

fn main() {
    rocket::ignite()
        .mount("/", routes![frontend::index])
        .attach(templates::Template::fairing())
        .launch();
}