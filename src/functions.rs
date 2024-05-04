use std::future::Future;

use reqwest::{header::HeaderMap, Method};
use crate::builder::Builder;

pub struct Function
{
    name: String,
    options: Option<FunctionOptions>,
    headers: Option<HeaderMap>
}

struct FunctionOptions
{
    pub body: Option<String>,
    pub method: Method,
}

pub struct FunctionResponse<T>
{
    pub status: u16,
    pub content: T
}



impl Function
{
    pub fn new<T>(name: T) -> Function where T : Into<String>
    {
        Function
        {
            name: name.into(),
            options: 
                Some(FunctionOptions{
                    body: None,
                    method: Method::POST,
                }
            ),
            headers: None
        }
    }

    pub fn body(&mut self, body: &str)
    {
        let opt: &mut FunctionOptions = self.options.as_mut().unwrap();
        opt.body = Some(String::from(body));
    }

    pub fn call(self, b: &Builder) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>>
    {
        let options: FunctionOptions = self.options.unwrap_or(
            FunctionOptions{body: None, method: Method::POST}
        );

        let mut req: reqwest::RequestBuilder = b.http_client.request(
            options.method, 
            format!("{}/{}", b.baseurl, self.name)
        );

        if options.body.is_some() 
        {
            req = req.body(options.body.unwrap());
        };

        req = req.headers(b.headers.clone());

        if self.headers.is_some() 
        {
            req = req.headers(self.headers.unwrap());
        }

        req.send()

    }

}

