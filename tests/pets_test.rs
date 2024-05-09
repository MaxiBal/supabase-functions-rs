use functions_rs::{Client, functions};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
enum Animal
{
    Dog,
    Cat,
}

#[derive(Debug, Deserialize)]
struct Pet
{
    name: String,
    animal: Animal,
    age: u16
}

#[derive(Deserialize)]
struct PetsResponse
{
    pets: Vec<Pet>
}

#[tokio::test]
async fn pets_test()
{

    let token = std::env::var("JWT").unwrap();
    let c: Client = Client::new("http://127.0.0.1:54321/functions/v1").auth(token);

    let function_res: functions::FunctionResponse<PetsResponse> = c.call("get-pets").await.unwrap();

    println!("Pets: {:?}", function_res.content.pets);

}