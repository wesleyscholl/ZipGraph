//! # ZipGraph Optimizer
//!
//! Query optimization and intelligent caching for graph operations.
//!
//! ## Features
//!
//! - Intelligent query planning
//! - Learned index structures
//! - Adaptive caching strategies
//! - Query result prediction
//! - Cost estimation
//!
//! ## Quick Start
//!
//! ```rust
//! use zipgraph_optimizer::QueryOptimizer;
//! use zipgraph_core::Graph;
//!
//! let mut optimizer = QueryOptimizer::new();
//! let graph = Graph::new();
//! 
//! // Optimizer learns and caches results
//! let path = optimizer.shortest_path(&graph, 0, 5);
//! ```

pub mod cache;
pub mod error;
pub mod optimizer;
pub mod query;

// Re-exports
pub use error::{OptimizerError, Result};
pub use optimizer::QueryOptimizer;
pub use query::{Query, QueryResult};
