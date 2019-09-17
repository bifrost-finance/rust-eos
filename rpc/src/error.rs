use failure::Fail;
use serde::{Deserialize, Serialize};
use std::fmt;


pub type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Parsed json error due to: {}", serde_err)]
    RequestJsonError {
        #[cause]
        serde_err: serde_json::Error,
    },
    #[fail(display = "Parsed json error due to: {}", serde_err)]
    ParseJsonError {
        #[cause]
        serde_err: serde_json::Error,
    },
    #[cfg(feature = "use-hyper")]
    #[fail(display = "Bad request due to: {}", request_err)]
    HttpRequestError {
        #[cause]
        request_err: hyper::Error,
    },
    #[fail(display = "No window error happened?")]
    NoWindow,
    #[cfg(feature = "use-hyper")]
    #[fail(display = "Bad http response due to: {}.", response_err)]
    HttpResponseError {
        #[cause]
        response_err: hyper::Error,
    },
    #[fail(display = "Bad http response due to: {}.", response_json_err)]
    ResponseJsonError {
        #[cause]
        response_json_err: serde_json::Error,
    },
    #[fail(display = "Bad http response due to: {:?}.", eos_err)]
    EosError{
        #[cause]
        eos_err: ErrorResponse,
    }
}

#[cfg(feature = "use-hyper")]
impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        println!("HYPER ERROR: {:#?}", err);
        Error::HttpRequestError { request_err: err }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        println!("SERDE ERROR: {:#?}", err);
        Error::ParseJsonError { serde_err: err }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
    pub error: ErrorMessage,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {:?}", self.code, self.message, self.error)
    }
}

impl std::error::Error for ErrorResponse {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorMessage {
    pub code: u32,
    pub name: String,
    pub what: String,
    pub details: Vec<ErrorDetails>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorDetails {
    pub message: String,
    pub file: String,
    pub line_number: u32,
    pub method: String,
}
