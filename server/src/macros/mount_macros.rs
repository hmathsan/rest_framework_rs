#[macro_export]
macro_rules! get {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::GET, $a.to_string(), $b)
    };
}

#[macro_export]
macro_rules! head {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::HEAD, $a.to_string(), $b)
    };
}

#[macro_export]
macro_rules! post {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::POST, $a.to_string(), $b)
    };
}

#[macro_export]
macro_rules! put {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::PUT, $a.to_string(), $b)
    };
}

#[macro_export]
macro_rules! delete {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::DELETE, $a.to_string(), $b)
    };
}

#[macro_export]
macro_rules! connect {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::CONNECT, $a.to_string(), $b)
    };
}

#[macro_export]
macro_rules! options {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::OPTIONS, $a.to_string(), $b)
    };
}

#[macro_export]
macro_rules! trace {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::TRACE, $a.to_string(), $b)
    };
}

#[macro_export]
macro_rules! patch {
    ($a:expr, $b:expr) => {
        (server::model::enums::method::Method::PATCH, $a.to_string(), $b)
    };
}
