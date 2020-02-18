use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, self::Error>;

#[derive(Debug)]
pub enum Error {
    EosError{
        eos_err: ErrorResponse,
    },
    #[cfg(feature = "use-hyper")]
    HttpRequestError {
        request_err: hyper::Error,
    },
    #[cfg(feature = "use-hyper")]
    HttpResponseError {
        response_err: hyper::Error,
    },
    #[cfg(feature = "use-hyper")]
    HyperTlsError {
        tls_err: hyper_tls::Error
    },
    #[cfg(feature = "use-hyper")]
    InvalidUri {
        invalid_uri: hyper::http::uri::InvalidUri,
    },
    NoWindow,
    RequestJsonError {
        serde_err: serde_json::Error,
    },
    ResponseJsonError {
        response_json_err: serde_json::Error,
    },
    ParseJsonError {
        serde_err: serde_json::Error,
    },
    #[cfg(feature = "use-hyper")]
    TokioError {
        tokio_err: tokio::io::Error,
    },
}

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::EosError{ref eos_err} => write!(f, "Bad eos http response due to: {:?}.", eos_err),
            Self::HttpRequestError{ref request_err} => write!(f, "Bad hyper request due to: {}.", request_err),
            Self::HttpResponseError{ref response_err} => write!(f, "Bad hyper response due to: {}.", response_err),
            Self::InvalidUri{ref invalid_uri} => write!(f, "Failed to parse uri due to: {}.", invalid_uri),
            Self::HyperTlsError{ref tls_err} => write!(f, "Failed to create tls connector due to: {}.", tls_err),
            Self::NoWindow => write!(f, "no window error."),
            Self::RequestJsonError{ref serde_err} => write!(f, "Invalid json object as request due to: {}.", serde_err),
            Self::ResponseJsonError{ref response_json_err} => write!(f, "Responsed an invalid json object due to: {}.", response_json_err),
            Self::ParseJsonError{ref serde_err} => write!(f, "Failed to parse json object due to: {}.", serde_err),
            Self::TokioError{ref tokio_err} => write!(f, "Failed to start a tokio runtime due to: {}.", tokio_err),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Self::EosError{..} => "Bad eos http response.",
            Self::HttpRequestError{..} => "Bad hyper request.",
            Self::HttpResponseError{..} => "Bad hyper response.",
            Self::InvalidUri{..} => "Failed to parse uri.",
            Self::HyperTlsError{..} => "Failed to create tls connector",
            Self::NoWindow => "no window error.",
            Self::RequestJsonError{..} => "Failed to parse json object.",
            Self::ResponseJsonError{..} => "Bad http response.",
            Self::ParseJsonError{..} => "Parsed json object error.",
            Self::TokioError{..} => "Failed to start a tokio runtime.",
        }
    }
}

#[cfg(feature = "use-hyper")]
impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::HttpRequestError { request_err: err }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseJsonError { serde_err: err }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
    pub error: ErrorMessage,
}

#[cfg(feature = "std")]
impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}", self.code, self.message, self.error)
    }
}

#[cfg(feature = "std")]
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
