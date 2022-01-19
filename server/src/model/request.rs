use std::collections::HashMap;

use serde::{Deserialize};

#[derive(Debug)]
pub struct Request<T> where T: Deserialize<'static> {
    path: String,
    headers: HashMap<String, String>,
    method: String,
    body: T
}

impl<T> Request<T> 
    where T: Deserialize<'static>
{
    pub fn new(path: String, method: String, headers: HashMap<String, String>, body: T) -> Self {
        Self {
            path,
            headers,
            method,
            body: body
        }
    }
}