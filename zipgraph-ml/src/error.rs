//! Error types for ZipGraph ML

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MlError {
    #[error("Model not trained")]
    ModelNotTrained,

    #[error("Invalid model configuration: {0}")]
    InvalidConfig(String),

    #[error("Training error: {0}")]
    TrainingError(String),

    #[error("Inference error: {0}")]
    InferenceError(String),

    #[error("Feature extraction error: {0}")]
    FeatureError(String),

    #[error("Graph error: {0}")]
    GraphError(#[from] zipgraph_core::GraphError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, MlError>;
