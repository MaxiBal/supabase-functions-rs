use reqwest::{
    header::{HeaderMap, HeaderValue}, Client
};


#[derive(Debug)]
pub struct Builder 
{
    pub baseurl: String,
    pub headers: HeaderMap,
    pub http_client: Client
}

impl Builder
{
    ///
    /// Creates a new `Builder` using a `url`.
    /// 
    pub fn new<T>(url: T,) -> Builder where T : Into<String>
    {
        let mut b = Builder
        {
            baseurl: url.into(),
            headers: HeaderMap::new(),
            http_client: Client::new()
        };

        b.headers.append("Content-Type", HeaderValue::from_bytes(b"application/json").unwrap());

        b
    }

    ///
    /// Authenticates the builder using JWT token `token`.
    /// 
    pub fn auth<T>(mut self, token: T) -> Self where T : Into<String>
    {
        self.headers.insert(
            "Authorization", 
            HeaderValue::from_str(&format!("Bearer {}", token.into())).unwrap()
        );
        self
    }
    


}
