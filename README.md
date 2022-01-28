# A simple REST Framework in Rust

This is a very basic REST Framework implementation that I've done for study and practice of the language.

It supports all the HTTP Methods, path and query parameters and uses [`serde`](https://serde.rs/) for Serializing and Deserializing JSON data.

## Examples

_P.S: The body buffer has a limited size of 10240 bytes, requests larger than that unfortunately will result in a panic. If I ever return to this project I will add a dotenv variable for requests sizes_

_P.S 2: As I said before, the project was created for practicing and study, so I didn't uploaded to `crates.io`, the only way to execute it is to clone it._

#

To get started and just start the server without any mounted endpoint just do the following:

Import the macros and necessary structs and enums:

```rust
#[macro_use] extern crate macros;
#[macro_use] extern crate server;

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use server::{server::Server, model::{enums::status_code::StatusCode, response_entity::{ResponseEntityBuilder, ResponseEntity}}};
```

Then on the main function create the server instance:

```rust
fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
```

If you run now you will see a simple message saying that the server is listening to `127.0.0.1:8080`. But if you try to do any request on this endpoint you will get the default error message saying that a function wasn't found, that's because we didn't mount any endpoints yet.

### Simple GET Hello Request

Now to mount a endpoint that returns a simple Hello to someone, just do the following.

We first create the Request and Responses structs:

```rust
#[derive(Request)]
#[request_obj]
struct HelloRequest {
    name: String
}

#[derive(Response)]
#[response_obj]
struct HelloResponse {
    message: String
}
```

_P.S: The `#[derive(Request)]`, `#[derive(Response)]`, `#[request_obj]` and `#[response_obj]` are necessary to generate the necessary code that implements the necessary traits so that the server recognizes what needs to be serializable or not_

Now we must create the function that gets executed when the request is made for that endpoint.

```rust
fn hello(_headers: HashMap<String, String>, _params: HashMap<String, String>, req: HelloRequest) -> ResponseEntity {
    ResponseEntityBuilder::new()
        .with_body(HelloResponse { message: format!("Hello, {}", req.name) })
        .with_status_code(StatusCode::Ok)
        .build()
}
```

#

So, that's a lot going on in here, let's explain one by one.

The `_headers: HashMap<String, String>` attribute contains all the headers the request has sent, we don't need it here but the server returns it for all the functions, so we simply add a `_` before the variable to tell rust the variable will not be used.

The `_params: HashMap<String, String>` attribute contains all the parameters received by the request either by the path or query. It will be empty because we haven't defined any path parameter and don't plan to send any query parameters either, so we add a `_` there too.

The `req: HelloRequest` contains the JSON body object sent by the request.

The return type `ResponseEntity` is default for every endpoint implementation, it implements the necessary traits to Serialize and Deserialize the body and method data. The only necessary attribute that must be informed is the `Method`, it doesn't need a body if you don't want to send it.

#

Now the only thing that we need to do is mount the desired endpoint and the function to the server.

There's a macro for every HTTP method to make it more readable to pass the necessary variables. For this example we will use the `get![]` macro.

```rust
fn main() {
    let mut server = Server::new("127.0.0.1:8080".to_string());
    server.mount(get!["/hello", hello]);
    server.run();
}
```

Now if you use Postman or Insomnia to call the endpoint `127.0.0.1:8080/hello` passing the body: 
```json
{ "name": "John" }
``` 
you will get this response:

```json
{
	"message": "Hello, John"
}
```

### Implementing more endpoints

Now if you want to implement more endpoints, just follow the exact same example as `/hello` but using the desired method macros.

## What I want to implement later

- [ ] Middleware support
- [ ] Dotenv support
- [ ] Logging