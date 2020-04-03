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
use pre_kan::error_handling;
use crate::lib_redis::*;

pub mod frontend;
pub mod lib_redis;

use std::string::String;

use rocket::request::Request;
use rocket::response::{ Response, Responder, Result };
use rocket::http::{ Status, ContentType };

use rocket_contrib::templates;
use rocket_contrib::json::{ Json, JsonValue };

use regex::Regex;
use serde::{ Serialize, Deserialize };
use rocket_client_addr::ClientRealAddr;

#[derive(Serialize, Deserialize)]
pub struct PlaceType {
    token: String,
    ip: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorType {
    error: String,
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
pub fn place(data: Json<PlaceType>, remote_addr: ClientRealAddr) -> ApiResponse {
    let check_string: u32 = String::from(&data.0.token).chars().count() as u32;
    let length: u32 = 40;

    let client_ipv4: String = remote_addr.get_ipv4_string().expect("check ipv6");
    let client_ipv6: String = remote_addr.get_ipv6_string();
    let client: String;

    let ip_error: String = String::from("your ip is something wrong...");
    let no_hack: String = String::from("no hack");

    if client_ipv4.is_empty() == true {
        client = client_ipv6
    }

    else {
        if client_ipv4 == String::from("127.0.0.1") {
            client = no_hack
        }

        else if client_ipv4 == String::from("localhost") {
            client = no_hack
        }

        else if client_ipv4 == String::from("0.0.0.0") {
            client = no_hack
        }

        else if client_ipv4 == String::from("8.8.8.8") {
            client = ip_error
        }

        else if client_ipv4 == String::from("1.1.1.1") {
            client = ip_error
        }

        else {
            client = client_ipv4
        }
    }

    let special_letter = Regex::new(r"[!@#$%^&*(),.?:{}|<>]").unwrap();
    let result: String = String::from(special_letter.replace_all(&data.0.token, "nohack"));

    let req_url: String = format!("http://{}/kcsapi/api_get_member/deck", &data.0.ip);
    let client_ip: String = format!("{}", &client);
    let client_ip_data: String = client_ip.clone();
    let token_data: String = data.0.token.clone();
    
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
                let client_ip_copy = client_ip.clone();
                let token_copy = data.0.token.clone();

                let result_data = format!("{:?}", post_data(req_url, data.0.token, client_ip));

                match connect_redis(client_ip_copy, token_copy) {
                    Ok(()) => {
                        status_point = String::from("success!");
                        data_status = String::from(result_data);
                    }

                    Err(_) => {
                        status_point = String::from("success, but not saved.");
                        data_status = String::from(result_data);
                    }
                }
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

    match lib_redis::connect_redis(client_ip_data, token_data) {
        Ok(()) => None,
        Err(_) => Some(5)
    };

    ApiResponse {
        json: json!({
            "status": &status_point,
            "data": &data_status
        }),
        status: Status::NotImplemented
    }
}

#[post("/api/v1/errors", format = "application/json", data = "<data>")]
pub fn error_control(data: Json<ErrorType>, remote_addr: &ClientRealAddr) -> ApiResponse {
    if data.0.error.is_empty() == true {
        ApiResponse {
            json: json! ({
                "status": "failed",
                "data": "no_data"
            }),
            status: Status::NotImplemented
        }
    }
    
    else {
        let special_letter = Regex::new(r"[!@#$%^&*(),.?:{}|<>]").unwrap();
        let result_error: String = String::from(special_letter.replace_all(&data.0.error, "nohack"));

        let client_ipv4: String = remote_addr.get_ipv4_string().expect("check ipv6");
        let client_ipv6: String = remote_addr.get_ipv6_string();
        let client: String;

        let ip_error: String = String::from("your ip is something wrong...");
        let no_hack: String = String::from("no hack");

        if client_ipv4.is_empty() == true {
            client = client_ipv6
        }

        else {
            if client_ipv4 == String::from("127.0.0.1") {
                client = no_hack
            }

            else if client_ipv4 == String::from("localhost") {
                client = no_hack
            }

            else if client_ipv4 == String::from("0.0.0.0") {
                client = no_hack
            }

            else if client_ipv4 == String::from("8.8.8.8") {
                client = ip_error
            }

            else if client_ipv4 == String::from("1.1.1.1") {
                client = ip_error
            }

            else {
                client = client_ipv4
            }
        }

        let result_ip: String = client.clone();

        error_handling(&establish_connection(), client, result_error);
        ApiResponse {
            json: json! ({
                "status": "success!",
                "data": &result_ip
            }),
            status: Status::Ok
        }
    }
}

#[tokio::main]
pub async fn post_data(urls: String, token: String, ip: String) -> std::result::Result<reqwest::blocking::Response, reqwest::Error> {
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
        .mount("/", routes![frontend::index, place, error_control, frontend::all_public])
        .attach(templates::Template::fairing())
        .launch();
}