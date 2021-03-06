use std::collections::HashMap;

use regex::Regex;

use crate::{model::{request::RequestObj, Request}, server::{Server, Endpoint}};

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
            }
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
            }
            BufferSteps::Json => {
                temp_vec = buffer[(i - 1)..(buffer.len() - 1)].to_vec();
                processed_vec.push(temp_vec.clone());

                break;
            }
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
            let filtered_end: Vec<(usize, &u8)> = buf
                .iter()
                .enumerate()
                .filter(|(_i, f)| **f == 125 as u8)
                .collect();
            let last_index: usize = filtered_end.iter().last().unwrap().0;

            body = String::from_utf8_lossy(&buf[0..last_index + 1]).to_string();
        }

        if i == 0 {
            let str = String::from_utf8_lossy(buf);

            str.split(' ')
                .into_iter()
                .enumerate()
                .for_each(|(i, f)| match i {
                    0 => method = f.to_string(),
                    1 => path = f.to_string(),
                    _ => {}
                });
        } else if i < processed_buf_size {
            let mut key = String::new();
            let mut value = String::new();

            String::from_utf8_lossy(buf)
                .split(':')
                .into_iter()
                .enumerate()
                .for_each(|(i, f)| match i {
                    0 => key = f.trim().to_string(),
                    1 => value = f.trim().to_string(),
                    _ => {}
                })
        }
    }

    RequestObj::new(path, method, headers, body)
}

impl<'s, Req> Server<Req> 
    where Req: Request 
{
    pub(in crate) fn parse_path_return_func(&self, endpoint: Endpoint) -> (Option<Endpoint>, HashMap<String, String>) {
        let mut path_vec = endpoint.path.clone();
        let method = endpoint.method.clone();

        let path_param_regex = Regex::new("\\{([^A-Z]*?)\\}").unwrap();

        let query_params: HashMap<String, String> = if path_vec.last().unwrap().contains('?') {
            parse_query_params(path_vec.last().unwrap().split('?').last().unwrap().to_string())
        } else { HashMap::new() };

        if path_vec.last().unwrap() == "" && path_vec.len() > 1 {
            path_vec.remove(path_vec.len() - 1);
        }

        let mut same_vec: Option<Endpoint> = None;
        let mut endpoint: Endpoint;

        let mut possible = false;
        let mut has_param = false;

        for (e, _value) in &self.funcs {
            if e.path.len() == path_vec.len() {
                endpoint = e.clone();
                for (i, s) in path_vec.iter().enumerate() {
                    let current_element = endpoint.path.get(i).unwrap();
                    if s != current_element {
                        if path_param_regex.is_match(current_element) {
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

                if e.method == method && possible {
                    same_vec = Some(endpoint);
                } else {
                    possible = false;
                }
            }
        }

        if let Some(e) = same_vec {
            let mut params: HashMap<String, String> = if let true = !query_params.is_empty() {
                query_params
            } else { HashMap::new() };
            
            if has_param {
                for (i, endpoint) in e.path.iter().enumerate() {
                    if path_param_regex.is_match(endpoint) {
                        params.insert(
                            endpoint.replace(&['{', '}'], ""),
                            path_vec.get(i).unwrap().to_string(),
                        );
                    }
                }
            }

            return (Some(e.clone()), params);
        }

        (None, HashMap::new())
    }

}

pub(in crate) fn parse_query_params(query: String) -> HashMap<String, String> {
    let query_parsed: Vec<String> = query.split('&')
        .map(|q| q.to_string()).collect();

    let mut query_map: HashMap<String, String> = HashMap::new();
    
    for q in query_parsed {
        let item: Vec<String> = q.split('=').map(|f| f.to_string()).collect();

        query_map.insert(item.first().unwrap().to_string(), item.last().unwrap().to_string());
    }

    query_map
}