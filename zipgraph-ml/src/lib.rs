//! # ZipGraph ML
//!
//! Machine learning components for intelligent graph optimization.
//!
//! ## Features
//!
//! - Node embeddings (Node2Vec, GraphSAGE)
//! - Algorithm selection using ML
//! - Anomaly detection in graphs
//! - Pattern learning and recognition
//! - Graph neural networks
//!
//! ## Quick Start
//!
//! ```rust
//! use zipgraph_ml::{AlgorithmSelector, NodeEmbeddings};
//! use zipgraph_core::Graph;
//!
//! let graph = Graph::new();
//! let selector = AlgorithmSelector::new();
//! let algorithm = selector.select(&graph);
//! ```

pub mod algorithm_selector;
pub mod anomaly;
pub mod embeddings;
pub mod error;
pub mod features;

// Re-exports
pub use algorithm_selector::AlgorithmSelector;
pub use anomaly::{Anomaly, AnomalyDetector};
pub use embeddings::NodeEmbeddings;
pub use error::{MlError, Result};
