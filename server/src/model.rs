use serde::{Deserialize, Serialize};

pub(in crate) mod request;
pub mod enums;
pub(in crate) mod response;
pub mod response_entity;

// TODO: Make pub in crate after creating proc_macro_derive
pub trait Request: Serialize + Deserialize<'static> + Clone { 
    fn string_body_to_obj(body: String) -> Self
        where Self: Serialize + Deserialize<'static> + Sized + Clone;
}
pub trait Response: Serialize { 
    fn to_string_json(&self) -> String;
}

