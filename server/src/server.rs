use std::{net::TcpListener, io::{Read, Write}, time::Instant, collections::HashMap};
use serde::{Deserialize, Serialize};

use crate::model::{request::*, response::*, enums::{status_code::*, parse_error::ParseError}};

pub trait Handler {
    fn handle_request<T>(&mut self, request: &Request<T>) -> Response<T> where T: Serialize + Deserialize<'static>;

    fn handle_bad_request<T>(&mut self, err: &ParseError) -> Response<T> where T: Serialize + Deserialize<'static> {
        println!("Failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self{ addr }
    }

    pub fn run(self) {
        println!("Listening to {}", self.addr);

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

                            if let Err(e) = write!(
                                stream,
                                "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\n\r\n{}",
                                200,
                                "Ok",
                                "{\r\n\"Ok\": \"Ok\"\r\n}"
                            ) {
                                println!("Failed to send response: {}", e);
                            }

                            println!("Elapsed time: {:.2?}", now.elapsed());
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

fn buffer_to_request(processed_buf: &Vec<Vec<u8>>) -> Request<String> {
    let mut path = String::new();
    let mut method = String::new();
    let headers: HashMap<String, String> = HashMap::new();
    let mut body = String::new();

    let processed_buf_size = &processed_buf.len() - 1;

    for (i, buf) in processed_buf.iter().enumerate() {
        
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
        } else {
            let filtered: Vec<(usize, &u8)> = buf.iter().enumerate().filter(|(_i, f)| **f == 125 as u8).collect();
            let last_index: usize = filtered.iter().last().unwrap().0;

            body = String::from_utf8_lossy(&buf[i..last_index]).to_string();
        }
    }

    Request::new(path, method, headers, body)
}