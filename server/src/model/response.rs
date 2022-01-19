use std::fmt::{Display, Formatter, Result};

use serde::{Serialize};

use super::enums::status_code::StatusCode;

pub struct Response<T> 
    where T: Serialize 
{
    status_code: StatusCode,
    body: Option<T>
}

impl<T> Response<T> 
    where T: Serialize 
{
    pub fn new(status_code: StatusCode, body: Option<T>) -> Self {
        Self {
            status_code,
            body
        }
    }
}

impl<T> Display for Response<T> 
    where T: Serialize
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let json_body: String = match &self.body {
            Some(b) => {
                serde_json::to_string_pretty(&b).unwrap()
            },
            None => "".to_string(),
        };
        write!(
            f, "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            json_body
        )
    }
}