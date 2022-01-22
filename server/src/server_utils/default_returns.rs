use std::{net::TcpStream, io::Write};

use serde_derive::{Serialize, Deserialize};

use crate::model::enums::{status_code::StatusCode, method::Method};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnBody {
    error: Option<String>,
    message: String,
    detail: String
}

impl Default for ReturnBody {
    fn default() -> Self {
        Self { 
            error: None, 
            message: String::from("No message available"), 
            detail: String::from("A unknown internal error ocurred.")
        }
    }
}

impl ReturnBody {
    pub fn new(error: Option<String>, message: String, detail: String) -> Self {
        Self { error, message, detail }
    }

    pub fn to_string_body(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug)]
pub struct DefaultReturns;

impl DefaultReturns {
    pub fn not_found(stream: &mut TcpStream, body: ReturnBody) {    
        if let Err(e) = write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\n\r\n{}",
            StatusCode::NotFound.status_number(),
            StatusCode::NotFound.reason_phrase(),
            body.to_string_body()
        ) {
            println!("Failed to send response: {}", e);
        }
    }

    pub fn internal_error(stream: &mut TcpStream, body: Option<ReturnBody>) {
        let body = body.unwrap_or(ReturnBody::default());

        if let Err(e) = write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\n\r\n{}",
            StatusCode::InternalServerError.status_number(),
            StatusCode::InternalServerError.reason_phrase(),
            body.to_string_body()
        ) {
            println!("Failed to send response: {}", e);
        }
    }

    pub fn func_not_found(stream: &mut TcpStream, method: Method, path: String) {
        let message = format!(
            "Function for method {method} and path {path} doesn't exist"
            ).to_string();
        let detail = format!(
            "Function for method {method} and path {path} does not exist or probably wasn't implemented yet"
            ).to_string();

        println!("{}", &message);
        println!("Returning default 404 message");

        Self::not_found(stream, ReturnBody::new(None, message, detail));
        
    }
}