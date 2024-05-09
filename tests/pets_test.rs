use functions_rs::{Client, functions};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Pet
{
    name: String,
    animal: u16,
    age: u16
}

#[derive(Deserialize, Debug)]
struct PetsResponse
{
    pets: Vec<Pet>
}


#[tokio::test]
async fn pets_test()
{
    
    let pets: Vec<Pet> = vec![
        Pet{name: String::from("Fido"), age: 6, animal: 0},
        Pet{name: String::from("Garfield"), age: 12, animal: 1},
        Pet{name: String::from("Whiskers"), age: 3, animal: 1},
        Pet{name: String::from("Bella"), age: 5, animal: 0},
        Pet{name: String::from("Shadow"), age: 4, animal: 1},
        Pet{name: String::from("Max"), age: 7, animal: 0},
        Pet{name: String::from("Luna"), age: 2, animal: 1},
        Pet{name: String::from("Charlie"), age: 8, animal: 0},
        Pet{name: String::from("Oliver"), age: 1, animal: 1},
        Pet{name: String::from("Daisy"), age: 5, animal: 0},
        Pet{name: String::from("Simba"), age: 3, animal: 1}
    ];

    let token = std::env::var("JWT").unwrap();
    let c: Client = Client::new("http://127.0.0.1:54321/functions/v1").auth(token);

    let function_res: functions::FunctionResponse<PetsResponse> = c.call("get-pets").await.unwrap();

    assert_eq!(pets, function_res.content.pets);
}