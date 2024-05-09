use std::error::Error;
use std::fmt::{Display, Formatter, Result};


#[derive(Debug)]
pub struct FunctionError
{
    function_name: String,
    http_status: u16,
}

impl Display for FunctionError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Calling function \"{}\" gave error code: {}", self.function_name, self.http_status)
    }
}


impl Error for FunctionError {}

impl FunctionError
{
    pub fn new(function_name:String, http_status: u16) -> FunctionError
    {
        FunctionError{
            function_name,
            http_status
        }
    }
}