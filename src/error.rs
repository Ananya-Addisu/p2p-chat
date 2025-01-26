use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("Network error: {0}")]
    NetworkError(#[from] std::io::Error),
    
    #[error("Encryption error: {0}")]
    CryptoError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
    
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    #[error("Peer error: {0}")]
    PeerError(String),
}

impl From<ring::error::Unspecified> for ChatError {
    fn from(_: ring::error::Unspecified) -> Self {
        ChatError::CryptoError("Ring unspecified error".into())
    }
}