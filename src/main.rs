#[macro_use] extern crate macros;
#[macro_use] extern crate server;

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use server::{server::Server, model::{enums::status_code::StatusCode, response_entity::{ResponseEntityBuilder, ResponseEntity}}};

fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.mount(get!["/teste/{id_teste}", test_get]);
    server.mount(post!["/teste/", teste_post]);
    server.run();
}

fn test_get(_headers: HashMap<String, String>, _params: HashMap<String, String>, req: TestRequest) -> ResponseEntity {
    ResponseEntityBuilder::new()
        .with_body(TestResponse{ 
            test: format!("O campo test do request é igual a {}", req.test) ,
            test_params: format!("Param id_teste recebido com o valor = {}", _params.get("id_teste").unwrap())
        })
        .with_status_code(StatusCode::Ok)
        .build()
}

fn teste_post(_headers: HashMap<String, String>, _params: HashMap<String, String>, _req: TestRequest) -> ResponseEntity {
    ResponseEntityBuilder::new()
        .with_status_code(StatusCode::Ok)
        .build()
}

#[derive(Request)]
#[request_obj]
struct TestRequest {
    test: String
}

#[derive(Response)]
#[response_obj]
struct TestResponse {
    test: String,
    test_params: String
}
