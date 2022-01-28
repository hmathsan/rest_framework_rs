#[macro_use] extern crate macros;
#[macro_use] extern crate server;

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use server::{server::Server, model::{enums::status_code::StatusCode, response_entity::{ResponseEntityBuilder, ResponseEntity}}};

fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.mount(get!["/hello", hello]);
    server.run();
}

fn hello(_headers: HashMap<String, String>, _params: HashMap<String, String>, req: HelloRequest) -> ResponseEntity {
    ResponseEntityBuilder::new()
        .with_body(HelloResponse { message: format!("Hello, {}", req.name) })
        .with_status_code(StatusCode::Ok)
        .build()
}

#[derive(Request)]
#[request_obj]
struct HelloRequest {
    name: String
}

#[derive(Response)]
#[response_obj]
struct HelloResponse {
    message: String
}
