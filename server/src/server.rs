use std::{net::TcpListener, io::Read, time::Instant, collections::HashMap};
use serde::{Deserialize, Serialize};

use crate::{model::{request::*, response::*, enums::{status_code::*, parse_error::ParseError, method::Method}, Request, response_entity::ResponseEntity}, server_utils::{server_utils::{process_buffer, buffer_to_request}, default_returns::{DefaultReturns, ReturnBody}}, };

pub trait Handler {
    fn handle_request<T>(&mut self, request: &RequestObj<T>) -> ResponseObj<T> where T: Serialize + Deserialize<'static>;

    fn handle_bad_request<T>(&mut self, err: &ParseError) -> ResponseObj<T> where T: Serialize + Deserialize<'static> {
        println!("Failed to parse request: {}", err);
        ResponseObj::new(StatusCode::BadRequest, None)
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub(in crate) struct Endpoint {
    pub(in crate) method: Method,
    pub(in crate) path: Vec<String>
}

impl Endpoint {
    fn new(method: Method, path: String) -> Self {
        let mut path_vec: Vec<String> = path.split("/").map(|p| p.to_string()).collect();

        if path_vec.last().unwrap() == "" && path_vec.len() > 1 {
            path_vec.remove(path_vec.len() - 1);
        }

        Self { method, path: path_vec }
    }
}

pub struct Server<Req> 
    where Req: Request
{ 
    pub(in crate) addr: String,
    pub(in crate) funcs: HashMap<Endpoint, fn(HashMap<String, String>, HashMap<String, String>, Req) -> ResponseEntity>
}

// TODO: add middleware support. Maybe have three macros, one for only a endpoint function, other for middleware and endpoint, and other for global middleware
impl<'s, Req> Server<Req> 
    where Req: Request
{
    pub fn new(addr: String) -> Self {
        Self { addr, funcs: HashMap::new() }
    }

    pub fn mount(&mut self, method: Method, path: String, func: fn(HashMap<String, String>, HashMap<String, String>, Req) -> ResponseEntity) {
        self.funcs.insert(Endpoint::new(method, path), func);
    }

    pub fn run(self) {
        println!("Listening to {}", self.addr);

        // TODO: TcpListener for each endpoint?
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // TODO: Change byte size based on dotenv
                    // 10485760
                    let mut buffer: Vec<u8> = [0_u8; 4096].to_vec();
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request received!");
                            let now = Instant::now();

                            let processed_buf = process_buffer(&buffer);

                            let request_obj = buffer_to_request(&processed_buf);
                            let path = request_obj.path.clone();
                            let method = request_obj.method.clone();

                            println!("Calling function for method {} and path {}", method, path);

                            let (func_key, params) = self.parse_path_return_func(&path);

                            println!("\n\n{:?} \n\n{:?}", func_key, params);

                            if let Some(k) = func_key {
                                let func = self.funcs.get(&k);

                                match func {
                                    Some(f) => {
                                        println!("Function found");
    
                                        let body = request_obj.body.clone();
                                        let return_obj = f(request_obj.headers, params, Request::string_body_to_obj(body.to_owned()));
    
                                        println!("Received ResponseEntity, returning");
    
                                        return_obj.write(&mut stream);
    
                                        println!("Elapsed time: {:?}", now.elapsed());
                                        continue;
                                    },
                                    None => {
                                        DefaultReturns::func_not_found(&mut stream, method, path);
    
                                        println!("Elapsed time: {:?}", now.elapsed());
                                        continue;
                                    },
                                }
                            } else {
                                DefaultReturns::func_not_found(&mut stream, method, path);
    
                                println!("Elapsed time: {:?}", now.elapsed());
                                continue;
                            }
                        },
                        Err(err) => {
                            println!("Failed to read from connection: {}", err);

                            DefaultReturns::internal_error(
                                &mut stream, 
                                Some(ReturnBody::new(
                                    None,
                                    String::from("A internal error ocurred while reading the request"),
                                    String::from(format!("{}", err))
                                ))
                            );
                        },
                    }
                },
                Err(err) => {
                    println!("Failed to stablish connection: {err}");
                    continue;
                },
            }
        }
    }

}
