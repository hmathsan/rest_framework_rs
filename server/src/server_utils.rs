use std::{net::TcpStream, io::Write, collections::HashMap};

use crate::model::{enums::status_code::StatusCode, request::RequestObj};

pub(in crate) fn return_default_404(mut stream: TcpStream, path: &String) {
    let formated_body = format!(
        "{{\r\n\t \"timestamp\": \"\",\t\"status\": 404,\t\"error\": \"Not Found\",\t\"path\": \"{path}\"}}",
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

pub(in crate) fn process_buffer(buffer: &Vec<u8>) -> Vec<Vec<u8>> {
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

pub(in crate) fn buffer_to_request(processed_buf: &Vec<Vec<u8>>) -> RequestObj<String> {
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