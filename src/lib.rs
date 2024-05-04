use std::future::Future;

use builder::Builder;
use functions::{Function, FunctionResponse};
use reqwest::Error;

mod builder;
mod functions;
mod exceptions;

pub struct Client
{
    builder: Builder,
}

impl Client
{
    pub fn new<T>(base_url: T) -> Client where T : Into<String>
    {
        Client { builder: Builder::new(base_url) }
    }

    pub fn auth<T>(mut self, token: T) -> Self where T : Into<String>
    {
       self.builder = self.builder.auth(token);
       self
    }

    pub fn call<T>(self, function_name: T) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> where T :Into<String> 
    {
        Function::new(function_name).call(&self.builder)
    }

    pub async fn call_with_body<'a, T, U>(self, function_name: T, function_data: &str) -> Result<FunctionResponse<U>, Error> where T :Into<String>, U : serde::de::Deserialize<'a>
    {
        let mut func = Function::new(function_name);
        func.body(function_data);
        let res: Result<reqwest::Response, reqwest::Error> = func.call(&self.builder).await;
        match res
        {
            Ok(response) => {
                
                let response_text = response.text().await.unwrap();

                Ok(FunctionResponse{
                    status: response.status().as_u16(),
                    content: serde_json::from_str(response_text.as_str()).unwrap()
                })
            },
            Err(err ) => {
                Err(err)
            }
        }
    }
}

