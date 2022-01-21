use std::{collections::HashMap, net::TcpStream, io::Write};

use serde::{Deserialize, Serialize};
use serde_derive::{Serialize, Deserialize};

use self::enums::status_code::StatusCode;

pub(in crate) mod request;
pub mod enums;
pub(in crate) mod response;

pub trait Request: Serialize + Deserialize<'static> + Clone { 
    fn string_body_to_obj(body: String) -> Self
        where Self: Serialize + Deserialize<'static> + Sized + Clone;
}
pub trait Response { 
    
}

#[derive(Clone)]
pub struct ResponseEntityBuilder {
    pub(in crate) body: Option<String>,
    pub(in crate) headers: HashMap<String, String>,
    pub(in crate) status: StatusCode
}

impl Default for ResponseEntityBuilder {
    fn default() -> Self {
        Self { body: None, headers: HashMap::new(), status: StatusCode::Ok }
    }
}

impl ResponseEntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_body<T>(self, body: T) -> Self
        where T: Serialize
    {
        let b = serde_json::to_string_pretty(&body).unwrap();
        Self {
            body: Some(b),
            headers: self.headers,
            status: self.status
        }
    }

    pub fn with_headers(self, headers: HashMap<String, String>) -> Self {
        Self {
            body: self.body,
            headers,
            status: self.status
        }
    }

    pub fn with_status_code(self, status: StatusCode) -> Self {
        Self {
            body: self.body,
            headers: self.headers,
            status
        }
    }

    pub fn build(&self) -> ResponseEntity {
        ResponseEntity::new(self.clone())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseEntity {
    pub(in crate) body: Option<String>,
    pub(in crate) headers: HashMap<String, String>,
    pub(in crate) status: StatusCode
}

impl ResponseEntity {
    pub(in crate) fn new(builder: ResponseEntityBuilder) -> Self {
        Self {
            body: builder.body,
            headers: builder.headers,
            status: builder.status
        }
    }

    pub(in crate) fn write(&self, stream: &mut TcpStream) {
        if let Err(e) = write!(
            stream,
            "HTTP/1.1 {} {}{}\r\n\r\n{}",
            self.status.status_number(),
            self.status.reason_phrase(),
            format_headers(self.headers.clone()),
            self.body.clone().unwrap_or("".to_string())
        ) {
            println!("Failed to send response: {}", e);
        }
    }
}

fn format_headers(hash_map: HashMap<String, String>) -> String {
    let mut headers: Vec<String> = vec![];

    for (key, value) in hash_map {
        headers.push(format!("{key}: {value}"));
    }

    let mut header_string = String::new();

    for header in headers {
        header_string.push_str(&format!("\r\n{header}"));
    }

    header_string
}