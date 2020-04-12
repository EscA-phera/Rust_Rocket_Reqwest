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

use std::string::String;
use serde::{ Serialize, Deserialize };

use rocket::request::Request;
use rocket::response::{ Response, Responder, Result };
use rocket::http::{ Status, ContentType };
use rocket_client_addr::ClientRealAddr;
use rocket_contrib::templates;
use rocket_contrib::json::{ Json, JsonValue };

use regex::Regex;
use reqwest::header::HeaderMap;

pub mod frontend;
pub mod lib_redis;

use pre_kan::create_connection;
use pre_kan::establish_connection;
use pre_kan::error_handling;
use crate::lib_redis::connect_redis;

#[derive(Serialize, Deserialize)]
pub struct PlaceType {
    token: String,
    ip: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    data: String,
    text: String,
    name: String,
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

    let ip_error: String = String::from("unknown ip players _ DNS");
    let no_hack: String = String::from("local players _ SET localhost");

    if client_ipv4.is_empty() == true { client = client_ipv6 }
    else {
        if client_ipv4 == String::from("127.0.0.1") { client = no_hack }
        else if client_ipv4 == String::from("localhost") { client = no_hack }
        else if client_ipv4 == String::from("0.0.0.0") { client = no_hack }
        else if client_ipv4 == String::from("8.8.8.8") { client = ip_error }
        else if client_ipv4 == String::from("1.1.1.1") { client = ip_error }
        else { client = client_ipv4 }
    }

    let special_letter = Regex::new(r"[!@#$%^&*(),.?:{}|<>]").unwrap();
    let result: String = String::from(special_letter.replace_all(&data.0.token, "nohack"));

    let req_url: String = format!("http://{}/kcsapi/api_get_member/preset_deck", &data.0.ip);
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

                let result_data: String = post_data(req_url, data.0.token, data.0.ip, data.0.version);

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
        status: Status::Ok
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

        if client_ipv4.is_empty() == true { client = client_ipv6 }
        else {
            if client_ipv4 == String::from("127.0.0.1") { client = no_hack }
            else if client_ipv4 == String::from("localhost") { client = no_hack }
            else if client_ipv4 == String::from("0.0.0.0") { client = no_hack }
            else if client_ipv4 == String::from("8.8.8.8") { client = ip_error }
            else if client_ipv4 == String::from("1.1.1.1") { client = ip_error }
            else if client_ipv4 == String::from("check ipv6") { client = client_ipv6 }
            else { client = client_ipv4 }
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
pub async fn post_data(urls: String, token: String, ip: String, version: String) -> String {
    let send_api = format!("api_verno=1&api_token={}", &token);
    let req = reqwest::Client::new();
    
    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::REFERER, reqwest::header::HeaderValue::from_str(format!("http://{}/kcs/port.swf?version={}", &ip, &version).as_str()).unwrap());
    headers.insert(reqwest::header::ACCEPT_LANGUAGE, reqwest::header::HeaderValue::from_static("ja,en-US;q=0.8,en;q=0.6"));
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Whale/1.0.39.11 Safari/537.36"));
    headers.insert(reqwest::header::ACCEPT_ENCODING, reqwest::header::HeaderValue::from_static("gzip,deflate,sdch"));
    headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"));

    let res = req.get(&urls)
        .headers(headers)
        .body(send_api);
        
    let check_response = res.send().await;
    create_connection(&establish_connection(), token, ip);

    match check_response {
        Ok(data) => {
            if data.status() == reqwest::StatusCode::OK {
                match data.text().await {
                    Ok(text) => String::from(format!("{}", text)),
                    Err(_) => String::from("there is no data")
                }
            }
            else { String::from("wrong data") }
        }
        Err(_) => String::from("failed to send")
    }
}

#[catch(404)]
pub fn no_page() -> templates::Template {
    let context = ErrorMessage {
        data: String::from("404"),
        text: String::from("PAGE NOT FOUND"),
        name: String::from("Page Not Found"),
    };

    templates::Template::render("error", &context)
}

#[catch(403)]
pub fn privilege() -> templates::Template {
    let context = ErrorMessage {
        data: String::from("404"),
        text: String::from("PAGE NOT FOUND"),
        name: String::from("Page Not Found")
    };

    templates::Template::render("error", &context)
}

#[catch(422)]
pub fn json_post_error() -> templates::Template {
    let context = ErrorMessage {
        data: String::from("404"),
        text: String::from("PAGE NOT FOUND"),
        name: String::from("Page Not Found")
    };

    templates::Template::render("error", &context)
}

#[catch(500)]
pub fn understand_error() -> templates::Template {
    let context = ErrorMessage {
        data: String::from("500"),
        text: String::from("INTERNAL SERVER ERROR"),
        name: String::from("Internal Server Error")
    };

    templates::Template::render("error", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![frontend::index, place, error_control, frontend::all_public])
        .register(catchers![no_page, privilege, understand_error, json_post_error])
        .attach(templates::Template::fairing())
        .launch();
}
