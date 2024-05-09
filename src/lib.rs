use builder::Builder;
use functions::{Function, FunctionResponse};
use reqwest::Error;
use serde::de::DeserializeOwned;

mod builder;
pub mod functions;

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

    async fn process_http_request<U>(self, res: Result<reqwest::Response, reqwest::Error>) 
        -> Result<FunctionResponse<U>, Error> where U : DeserializeOwned
    {
        match res
        {
           Ok(response) => {

               let status_code: u16 = response.status().as_u16();
               let response_text: String = response.text().await?;

               let deserialized_obj : Result<U, serde_json::Error> = serde_json::from_str::<U>(&response_text);

               match deserialized_obj
               {
                   Ok(obj) => {
                       Ok(FunctionResponse{
                           status: status_code,
                           content: obj
                       })
                   },
                   Err(err) =>
                   {
                       panic!("Could not deserialize object, err: {}.", err);
                   }
               }

               
           },
           Err(err ) => {
               Err(err)
           }
       }
    }

    pub async fn call<T, U>(self, function_name: T) -> Result<FunctionResponse<U>, Error> where T :Into<String>, U : DeserializeOwned
    {
       let res = Function::new(function_name).call(&self.builder).await;

       self.process_http_request(res).await
       
    }

    pub async fn call_with_body<T, U, V>(self, function_name: T, function_data: V) 
        -> Result<FunctionResponse<U>, Error> 
        where T :Into<String>, U : serde::de::DeserializeOwned, V : serde::ser::Serialize
    {
        let mut func = Function::new(function_name);

        let func_body = &serde_json::to_value(function_data).unwrap_or_default().to_string();

        func.body(func_body);

        let res: Result<reqwest::Response, reqwest::Error> = func.call(&self.builder).await;

        self.process_http_request(res).await
    }
}

