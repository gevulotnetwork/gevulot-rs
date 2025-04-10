/// Error types used throughout the Gevulot client library.
/// 
/// This module defines the central [`Error`] type that encapsulates all possible
/// errors that can occur when interacting with the Gevulot network, as well as 
/// conversions from various underlying error types.
use cosmrs::ErrorReport;
use hex::FromHexError;
use prost::{DecodeError, EncodeError};

/// Comprehensive error type for the Gevulot client library.
/// 
/// This enum represents all possible errors that can occur when interacting with
/// the Gevulot network, including network errors, encoding/decoding errors,
/// transaction errors, and more.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Indicates that a required attribute was missing in an event.
    /// 
    /// This error occurs when parsing blockchain events and a required
    /// attribute is not present in the event data.
    /// 
    /// # Parameters
    /// * `&'static str` - The name of the missing attribute.
    #[error("missing event attribute: {0}")]
    MissingEventAttribute(&'static str),
    
    /// Indicates that an event attribute had an invalid value.
    /// 
    /// This error occurs when an attribute in an event has a value that
    /// cannot be properly parsed or is otherwise invalid.
    /// 
    /// # Parameters
    /// * `&'static str` - The name of the invalid attribute.
    #[error("invalid event attribute: {0}")]
    InvalidEventAttribute(&'static str),
    
    /// Indicates that an unknown event kind was encountered.
    /// 
    /// This error occurs when trying to process an event with a type
    /// that is not recognized by the client.
    /// 
    /// # Parameters
    /// * `String` - The unrecognized event kind identifier.
    #[error("unknown event kind: {0}")]
    UnknownEventKind(String),
    
    /// Indicates an error occurred during RPC communication.
    /// 
    /// This error occurs when there is a problem with the connection to
    /// the Gevulot node or when an RPC call fails.
    /// 
    /// # Parameters
    /// * `String` - A description of the RPC connection error.
    #[error("rpc connection error: {0}")]
    RpcConnectionError(String),
    
    /// Indicates an error occurred when decoding data.
    /// 
    /// This error occurs when the client fails to decode data received
    /// from the Gevulot network, such as protobuf messages.
    /// 
    /// # Parameters
    /// * `String` - A description of the decode error.
    #[error("decode error: {0}")]
    DecodeError(String),
    
    /// Indicates an error occurred when encoding data.
    /// 
    /// This error occurs when the client fails to encode data to be sent
    /// to the Gevulot network, such as when constructing transaction messages.
    /// 
    /// # Parameters
    /// * `String` - A description of the encode error.
    #[error("encode error: {0}")]
    EncodeError(String),
    
    /// Indicates that a requested resource was not found.
    /// 
    /// This error occurs when querying for entities (like workers, pins, tasks)
    /// that do not exist in the Gevulot network.
    #[error("not found")]
    NotFound,
    
    /// Indicates an error occurred when parsing data.
    /// 
    /// This error occurs when the client fails to parse textual data into
    /// the expected format, such as when parsing addresses or hexadecimal values.
    /// 
    /// # Parameters
    /// * `String` - A description of the parse error.
    #[error("parse error: {0}")]
    Parse(String),
    
    /// Indicates an error from the underlying Tendermint client.
    /// 
    /// This error is a wrapper around errors from the Tendermint client,
    /// which is used for blockchain communication.
    /// 
    /// # Parameters
    /// * `tendermint::Error` - The underlying Tendermint error.
    #[error("tendermint error: {0}")]
    Tendermint(#[from] tendermint::Error),
    
    /// Indicates a transaction failed when submitted to the blockchain.
    /// 
    /// This error occurs when a transaction is rejected by the blockchain,
    /// such as when it fails validation or execution.
    /// 
    /// # Parameters
    /// * `String` - The transaction hash or identifier.
    /// * `u32` - The error code returned by the blockchain.
    /// * `String` - The error message describing why the transaction failed.
    #[error("tx {0} failed with code {1}: {2}")]
    Tx(String, u32, String),
    
    /// A catch-all for errors that don't fit into other categories.
    /// 
    /// # Parameters
    /// * `String` - A description of the unknown error.
    #[error("unknown error: {0}")]
    Unknown(String),
}

/// A type alias for Results that may contain a Gevulot client [`Error`].
/// 
/// This alias is used throughout the library to provide a consistent
/// error type for all operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Converts a boxed standard error into a Gevulot client [`Error`].
/// 
/// This implementation allows any boxed error to be converted into the
/// library's error type, typically as an [`Error::Unknown`].
impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Unknown(error.to_string())
    }
}

/// Converts a Protobuf encode error into a Gevulot client [`Error`].
/// 
/// This implementation handles errors that occur when encoding Protobuf
/// messages for transmission to the Gevulot network.
impl From<EncodeError> for Error {
    fn from(error: EncodeError) -> Self {
        Error::EncodeError(error.to_string())
    }
}

/// Converts a Cosmos RPC error into a Gevulot client [`Error`].
/// 
/// This implementation handles errors from the Cosmos SDK RPC client,
/// which is used for communicating with the Gevulot blockchain.
impl From<cosmrs::rpc::error::Error> for Error {
    fn from(error: cosmrs::rpc::error::Error) -> Self {
        Error::RpcConnectionError(error.to_string())
    }
}

/// Converts a gRPC status error into a Gevulot client [`Error`].
/// 
/// This implementation handles status errors from gRPC calls to the
/// Gevulot network, typically indicating RPC failures.
impl From<tonic::Status> for Error {
    fn from(error: tonic::Status) -> Self {
        Error::RpcConnectionError(error.to_string())
    }
}

/// Converts an HTTP URI error into a Gevulot client [`Error`].
/// 
/// This implementation handles errors that occur when parsing or validating
/// URIs for connecting to the Gevulot network.
impl From<http::uri::InvalidUri> for Error {
    fn from(error: http::uri::InvalidUri) -> Self {
        Error::RpcConnectionError(error.to_string())
    }
}

/// Converts a Protobuf decode error into a Gevulot client [`Error`].
/// 
/// This implementation handles errors that occur when decoding Protobuf
/// messages received from the Gevulot network.
impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Self {
        Error::DecodeError(error.to_string())
    }
}

/// Converts a gRPC transport error into a Gevulot client [`Error`].
/// 
/// This implementation handles network transport errors that occur during
/// gRPC communication with the Gevulot network.
impl From<tonic::transport::Error> for Error {
    fn from(error: tonic::transport::Error) -> Self {
        Error::RpcConnectionError(error.to_string())
    }
}

/// Converts a BIP32 key derivation error into a Gevulot client [`Error`].
/// 
/// This implementation handles errors that occur during cryptographic key
/// derivation, which is used for wallet and signing operations.
impl From<bip32::Error> for Error {
    fn from(error: bip32::Error) -> Self {
        Error::Parse(error.to_string())
    }
}

/// Converts a Cosmos SDK error report into a Gevulot client [`Error`].
/// 
/// This implementation handles errors from the Cosmos SDK, which underpins
/// the Gevulot blockchain implementation.
impl From<ErrorReport> for Error {
    fn from(error: ErrorReport) -> Self {
        Error::Unknown(error.to_string())
    }
}

/// Converts a hex parsing error into a Gevulot client [`Error`].
/// 
/// This implementation handles errors that occur when parsing hexadecimal
/// strings, which are common in blockchain addresses and identifiers.
impl From<FromHexError> for Error {
    fn from(error: FromHexError) -> Self {
        Error::Parse(error.to_string())
    }
}

/// Converts a string error into a Gevulot client [`Error`].
/// 
/// This implementation provides a convenient way to create an error from
/// a string message, typically for ad-hoc error conditions.
impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Unknown(error.to_string())
    }
}
