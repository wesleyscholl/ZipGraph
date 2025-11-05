//! Error types for ZipGraph Optimizer

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptimizerError {
    #[error("Query optimization failed: {0}")]
    OptimizationFailed(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Query execution failed: {0}")]
    ExecutionError(String),

    #[error("Graph error: {0}")]
    GraphError(#[from] zipgraph_core::GraphError),

    #[error("ML error: {0}")]
    MlError(#[from] zipgraph_ml::MlError),
}

pub type Result<T> = std::result::Result<T, OptimizerError>;
