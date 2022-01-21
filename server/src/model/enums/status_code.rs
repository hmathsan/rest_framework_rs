use std::fmt::{Display, Formatter, Result as FmtResult};

use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "BadRequest",
            Self::NotFound => "NotFound"
        }
    }

    pub fn status_number(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::BadRequest => 400,
            StatusCode::NotFound => 404,
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
