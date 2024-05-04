use functions_rs::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HelloMessage
{
    message: String
}

#[tokio::test]
async fn get_hello_res()
{
    let token = "[token]";
    let c: Client = Client::new("http://127.0.0.1:54321/functions/v1").auth(token);
    
    let f = c.call_with_body("hello", "{\"name\":\"Function\"}").await.unwrap();

    

    println!("Status Code: {}, Message: {}", f.status, f.content["message"]);

    assert_eq!("Hello Function!", f.content["message"]);
}