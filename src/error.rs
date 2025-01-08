use cosmrs::ErrorReport;
use hex::FromHexError;
use prost::{DecodeError, EncodeError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing event attribute: {0}")]
    MissingEventAttribute(&'static str),
    #[error("invalid event attribute: {0}")]
    InvalidEventAttribute(&'static str),
    #[error("unknown event kind: {0}")]
    UnknownEventKind(String),
    #[error("rpc connection error: {0}")]
    RpcConnectionError(String),
    #[error("decode error: {0}")]
    DecodeError(String),
    #[error("encode error: {0}")]
    EncodeError(String),
    #[error("not found")]
    NotFound,
    #[error("parse error: {0}")]
    Parse(String),
    #[error("tendermint error: {0}")]
    Tendermint(#[from] tendermint::Error),
    #[error("tx {0} failed with code {1}: {2}")]
    Tx(String, u32, String),
    #[error("unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Unknown(error.to_string())
    }
}

impl From<EncodeError> for Error {
    fn from(error: EncodeError) -> Self {
        Error::EncodeError(error.to_string())
    }
}

impl From<cosmrs::rpc::error::Error> for Error {
    fn from(error: cosmrs::rpc::error::Error) -> Self {
        Error::RpcConnectionError(error.to_string())
    }
}

impl From<tonic::Status> for Error {
    fn from(error: tonic::Status) -> Self {
        Error::RpcConnectionError(error.to_string())
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(error: http::uri::InvalidUri) -> Self {
        Error::RpcConnectionError(error.to_string())
    }
}

impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Self {
        Error::DecodeError(error.to_string())
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(error: tonic::transport::Error) -> Self {
        Error::RpcConnectionError(error.to_string())
    }
}

impl From<bip32::Error> for Error {
    fn from(error: bip32::Error) -> Self {
        Error::Parse(error.to_string())
    }
}

impl From<ErrorReport> for Error {
    fn from(error: ErrorReport) -> Self {
        Error::Unknown(error.to_string())
    }
}

impl From<FromHexError> for Error {
    fn from(error: FromHexError) -> Self {
        Error::Parse(error.to_string())
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Unknown(error.to_string())
    }
}
