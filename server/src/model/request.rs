use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize};

use crate::model::enums::method::*;

#[derive(Debug)]
pub struct RequestObj<T> 
    where T: Deserialize<'static> 
{
    pub path: String,
    pub headers: HashMap<String, String>,
    pub method: Method,
    pub body: T
}

impl<T> RequestObj<T> 
    where T: Deserialize<'static>
{
    pub(in crate) fn new(path: String, method: String, headers: HashMap<String, String>, body: T) -> Self {
        Self {
            path,
            headers,
            method: Method::from_str(&method).unwrap(),
            body: body
        }
    }

}