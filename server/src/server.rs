use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, time::Instant, collections::HashMap};
use serde::{Deserialize, Serialize};

use crate::model::{request::*, response::*, enums::{status_code::*, parse_error::ParseError, method::Method}, Request, ResponseEntity};

pub trait Handler {
    fn handle_request<T>(&mut self, request: &RequestObj<T>) -> ResponseObj<T> where T: Serialize + Deserialize<'static>;

    fn handle_bad_request<T>(&mut self, err: &ParseError) -> ResponseObj<T> where T: Serialize + Deserialize<'static> {
        println!("Failed to parse request: {}", err);
        ResponseObj::new(StatusCode::BadRequest, None)
    }
}

#[derive(Hash, PartialEq, Eq)]
pub(in crate) struct Endpoint {
    method: Method,
    path: String
}

impl Endpoint {
    fn new(method: Method, path: String) -> Self {
        Self{ method, path }
    }
}

pub struct Server<Req> 
    where Req: Request
{ 
    pub(in crate) addr: String,
    pub(in crate) funcs: HashMap<Endpoint, fn(HashMap<String, String>, Req) -> ResponseEntity>
}

impl<'s, Req> Server<Req> 
    where Req: Request
{
    pub fn new(addr: String) -> Self {
        Self { addr, funcs: HashMap::new() }
    }

    pub fn mount(&mut self, method: Method, path: String, func: fn(HashMap<String, String>, Req) -> ResponseEntity) {
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
                            let func = self.funcs.get(&Endpoint::new(method.clone(), path.clone()));

                            match func {
                                Some(f) => {
                                    println!("Function found");

                                    let body = request_obj.body.clone();
                                    let return_obj = f(request_obj.headers, Request::string_body_to_obj(body.to_owned()));

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
}

fn return_default_404(mut stream: TcpStream, path: &String) {
    let formated_body = format!("{{\r\n\t \"timestamp\": \"\",\t\"status\": 404,\t\"error\": \"Not Found\",\t\"path\": \"{path}\"}}",
        
    );

    if let Err(e) = write!(
        stream,
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\n\r\n{}",
        StatusCode::NotFound.status_number(),
        StatusCode::NotFound.reason_phrase(),
        formated_body
    ) {
        println!("Failed to send response: {}", e);
    }
}

fn process_buffer(buffer: &Vec<u8>) -> Vec<Vec<u8>> {
    enum BufferSteps {
        FindingSpace,
        SpaceFound,
        Json,
    }
    
    let mut processed_vec: Vec<Vec<u8>> = vec![];
    let mut temp_vec: Vec<u8> = vec![];
    
    let mut current_step = BufferSteps::FindingSpace;
    
    for (i, byte) in buffer.iter().enumerate() {
        match current_step {
            BufferSteps::FindingSpace => {
                if *byte == 13 as u8 && buffer[i + 1] == 10 as u8 {
                    current_step = BufferSteps::SpaceFound;
                } else if *byte == 123 {
                    current_step = BufferSteps::Json;
                } else {
                    temp_vec.push(*byte)
                }
            },
            BufferSteps::SpaceFound => {
                if temp_vec.len() > 0 {
                    processed_vec.push(temp_vec.clone());
                }
                
                if *byte != 13 as u8 && *byte != 10 as u8 {
                    if *byte == 123 {
                        temp_vec.push(*byte);
                        current_step = BufferSteps::Json;
                        continue;
                    } else {
                        temp_vec.push(*byte);
                        current_step = BufferSteps::FindingSpace;
                        continue;
                    }
                }
                
                temp_vec.clear();
            },
            BufferSteps::Json => {
                temp_vec = buffer[(i - 1)..(buffer.len() - 1)].to_vec();
                processed_vec.push(temp_vec.clone());
                
                break;
            },
        }
    }
    
    return processed_vec;
}

fn buffer_to_request(processed_buf: &Vec<Vec<u8>>) -> RequestObj<String> {
    let mut path = String::new();
    let mut method = String::new();
    let headers: HashMap<String, String> = HashMap::new();
    let mut body = String::new();

    let processed_buf_size = &processed_buf.len() - 1;

    for (i, buf) in processed_buf.iter().enumerate() {
        
        if i == processed_buf.len() - 1 {
            let filtered_end: Vec<(usize, &u8)> = buf.iter().enumerate().filter(|(_i, f)| **f == 125 as u8).collect();
            let last_index: usize = filtered_end.iter().last().unwrap().0;

            body = String::from_utf8_lossy(&buf[0..last_index + 1]).to_string();
        }

        if i == 0 {
            let str = String::from_utf8_lossy(buf);

            str.split(' ').into_iter().enumerate().for_each(|(i, f)| {
                match i {
                    0 => method = f.to_string(),
                    1 => path = f.to_string(),
                    _ => {}
                }
            });
        } else if i < processed_buf_size {
            let mut key = String::new();
            let mut value = String::new();

            String::from_utf8_lossy(buf).split(':').into_iter().enumerate().for_each(|(i, f)| {
                match i {
                    0 => key = f.trim().to_string(),
                    1 => value = f.trim().to_string(),
                    _ => {}
                }
            })
        }
    }

    RequestObj::new(path, method, headers, body)
}