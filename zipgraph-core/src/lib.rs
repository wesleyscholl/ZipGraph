//! # ZipGraph Core
//!
//! Core graph data structures and algorithms for the ZipGraph engine.
//!
//! ## Features
//!
//! - High-performance graph data structures
//! - Classic graph algorithms (BFS, DFS, Dijkstra, A*, PageRank)
//! - Graph statistics and analysis
//! - Efficient serialization and I/O
//! - Thread-safe operations
//!
//! ## Quick Start
//!
//! ```rust
//! use zipgraph_core::{Graph, Algorithm};
//!
//! let mut graph = Graph::new();
//! graph.add_edge(0, 1, 1.0);
//! graph.add_edge(1, 2, 2.0);
//!
//! let path = graph.shortest_path(0, 2, Algorithm::Dijkstra);
//! ```

pub mod algorithms;
pub mod error;
pub mod graph;
pub mod stats;
pub mod types;

// Re-exports for convenience
pub use error::{GraphError, Result};
pub use graph::{Edge, Graph, Node};
pub use stats::GraphStats;
pub use types::NodeId;

/// Algorithm selection enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    BFS,
    DFS,
    Dijkstra,
    AStar,
    BidirectionalSearch,
    FloydWarshall,
    BellmanFord,
}
