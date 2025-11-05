//! Error types for ZipGraph Core

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Node {0} not found")]
    NodeNotFound(usize),

    #[error("Edge from {0} to {1} not found")]
    EdgeNotFound(usize, usize),

    #[error("Invalid graph structure: {0}")]
    InvalidStructure(String),

    #[error("Algorithm error: {0}")]
    AlgorithmError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type Result<T> = std::result::Result<T, GraphError>;
