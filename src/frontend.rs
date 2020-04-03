extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_client_addr;

use rocket::response::NamedFile;
use rocket_contrib::templates;
use rocket_client_addr::ClientRealAddr;

use std::collections::HashMap;
use std::net::AddrParseError;
use std::path::{ Path, PathBuf };

#[get("/")]
pub fn index(ip: &ClientRealAddr) -> std::result::Result<templates::Template, AddrParseError> {
    let mut context = HashMap::new();

    let client_ipv4: String = ip.get_ipv4_string().expect("expected ipv6.");
    let client_ipv6: String = ip.get_ipv6_string();

    let ip_error: String = String::from("your ip is something wrong...");
    let no_hack: String = String::from("no hack");

    let client: String;

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

    context.insert("data", String::from(client));
    serde::export::Ok(templates::Template::render("index", &context))
}

#[get("/src/<path..>")]
pub fn all_public(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("src/").join(path)).ok()
}