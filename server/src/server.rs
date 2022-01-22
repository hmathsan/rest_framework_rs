use std::{net::TcpListener, io::Read, time::Instant, collections::HashMap};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{model::{request::*, response::*, enums::{status_code::*, parse_error::ParseError, method::Method}, Request, response_entity::ResponseEntity}, server_utils::{return_default_404, process_buffer, buffer_to_request}};

pub trait Handler {
    fn handle_request<T>(&mut self, request: &RequestObj<T>) -> ResponseObj<T> where T: Serialize + Deserialize<'static>;

    fn handle_bad_request<T>(&mut self, err: &ParseError) -> ResponseObj<T> where T: Serialize + Deserialize<'static> {
        println!("Failed to parse request: {}", err);
        ResponseObj::new(StatusCode::BadRequest, None)
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub(in crate) struct Endpoint {
    method: Method,
    path: Vec<String>
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
                                        println!("Function for method {} and path {} doesn't exist", method, path);
                                        println!("Returning default 404 message");
    
                                        return_default_404(stream, &request_obj.path);
    
                                        println!("Elapsed time: {:?}", now.elapsed());
                                        continue;
                                    },
                                }
                            } else {
                                println!("Function for method {} and path {} doesn't exist", method, path);
                                println!("Returning default 404 message");

                                return_default_404(stream, &request_obj.path);

                                println!("Elapsed time: {:?}", now.elapsed());
                                continue;
                            }

                        },
                        Err(err) => {
                            println!("Failed to read from connection: {}", err);
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

    fn parse_path_return_func(&self, path: &String) -> 
        (Option<Endpoint>, HashMap<String, String>) 
    {
        let path_param_regex = Regex::new("\\{([^A-Z]*?)\\}").unwrap();

        let mut path_vec: Vec<String> = path.split("/").map(|p| p.to_string()).collect();
        
        if path_vec.last().unwrap() == "" && path_vec.len() > 1 {
            path_vec.remove(path_vec.len() - 1);
        }

        let mut same_vec: Option<Endpoint> = None;
        let mut endpoint: Endpoint;

        let mut possible = false;
        let mut has_param = false;

        for (e, _value) in &self.funcs {
            println!("for loop");
            println!("{:?} {:?}", e, path_vec);
            if e.path.len() == path_vec.len() {
                println!("same size");
                endpoint = e.clone();
                for (i, s) in path_vec.iter().enumerate() {
                    println!("entrou");
                    let current_element = endpoint.path.get(i).unwrap();
                    if s != current_element {
                        if path_param_regex.is_match(current_element) {
                            println!("regex match");
                            possible = true;
                            has_param = true;
                        } else {
                            possible = false;
                            has_param = false;
                        }
                    } else {
                        possible = true;
                    }
                }

                if possible {
                    same_vec = Some(endpoint);
                }
            }
        }

        if let Some(e) = same_vec {
            let mut params: HashMap<String, String> = HashMap::new();
            if has_param {
                for (i, endpoint) in e.path.iter().enumerate() {
                    if path_param_regex.is_match(endpoint) {
                        params.insert(endpoint.replace(&['{', '}'], ""), path_vec.get(i).unwrap().to_string());
                    }
                }
            }

            return (Some(e.clone()), params)
        }
        
        (None, HashMap::new())
    }
}
