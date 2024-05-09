# functions-rs

Strongly typed client library for Supabase edge functions in rust.

## Usage

To use, initialize a client with the function url:

```rs
use functions_rs::Client;

...

let client = Client::new(env!("SUPABASE_FUNCTION_URL"));
```

From there, a function can be called using the ```call``` and ```call_with_body``` methods.

```rs
use functions_rs::functions;

#[derive(serde::Deserialize)]
struct HelloResponse
{
    message: String
}

...

let response: FunctionResponse<HelloResponse> = client.call("hello").await.unwrap();

println!("{}", response.content.message);
```

Or:


```rs
use functions_rs::functions;

#[derive(serde::Serialize)]
struct HelloRequest
{
    name: String
}

#[derive(serde::Deserialize)]
struct HelloResponse
{
    message: String
}

...

let request = HelloRequest{name: String::from("World")};

let response: FunctionResponse<HelloResponse> = client.call_with_body("hello", request).await.unwrap();

println!("{}", response.content.message);
```

To authenticate a client using a JWT, call the authenticate method on the client.

```rs
let token = env!("JWT");

let client = Client::new(env!("SUPABASE_FUNCTION_URL")).auth(token);
```
