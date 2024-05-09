use functions_rs::Client;
use functions_rs::functions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HelloMessage
{
    message: String
}

#[derive(Debug, Serialize)]
struct HelloRequest
{
    name: String
}

#[tokio::test]
async fn get_hello_res()
{
    let token = std::env::var("JWT").unwrap();
    let c: Client = Client::new("http://127.0.0.1:54321/functions/v1").auth(token);

    let req: HelloRequest = HelloRequest{name: String::from("Function")};
    
    let f: functions::FunctionResponse<HelloMessage> = 
        c.call_with_body("hello", &serde_json::to_value(req).unwrap().to_string()).await.unwrap();

    println!("Status Code: {}, Message: {}", f.status, f.content.message);

    assert_eq!("Hello Function!", f.content.message);
}