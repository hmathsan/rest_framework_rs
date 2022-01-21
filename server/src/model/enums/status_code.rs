use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatusCode {
    Ok,
    Created,
    Accepted,
    Unauthorized,
    Forbidden,
    BadRequest,
    NotFound,
    InternalServerError,
    Other((u16, String))
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::BadRequest => "BadRequest",
            Self::NotFound => "NotFound",
            Self::InternalServerError => "Internal Server Error",
            Self::Other((_code, phrase)) => phrase
        }
    }

    pub fn status_number(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
            Self::Other((code, _phrase)) => *code
        }
    }
}