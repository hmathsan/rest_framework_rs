use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use server::{server::Server, model::{enums::{method::Method, status_code::StatusCode}, Request, Response, response_entity::{ResponseEntityBuilder, ResponseEntity}}};

fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.mount(Method::GET, "/".to_string(), test_get);
    server.run();
}

fn test_get(_headers: HashMap<String, String>, req: TestRequest) -> ResponseEntity {
    ResponseEntityBuilder::new()
        .with_body(TestResponse{ test: format!("O campo test do request é igual a {}", req.test) })
        .with_status_code(StatusCode::Ok)
        .build()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TestRequest {
    test: String
}

#[derive(Serialize, Deserialize)]
struct TestResponse {
    test: String
}

impl Request for TestRequest {
    fn string_body_to_obj(body: String) -> Self
        where Self: serde::Serialize + serde::Deserialize<'static> + Sized + Clone {
        let b = &body[..];
        serde_json::from_str(b).unwrap()
    }
}

impl Response for TestResponse {
    fn to_string_json(&self) -> String {
        serde_json::to_string_pretty(self.clone()).unwrap()
    }
}
