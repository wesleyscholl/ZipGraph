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
//! use zipgraph_core::Graph;
//!
//! let mut graph = Graph::new();
//! graph.add_node_simple("A");
//! graph.add_node_simple("B");
//! graph.add_edge(0, 1, 1.0).unwrap();
//!
//! assert_eq!(graph.node_count(), 2);
//! assert_eq!(graph.edge_count(), 1);
//! ```

pub mod algorithms;
pub mod centrality;
pub mod error;
pub mod graph;
pub mod metrics;
pub mod parallel;
pub mod stats;
pub mod storage;
pub mod types;
pub mod ultra;

// Re-exports for convenience
pub use error::{GraphError, Result};
pub use graph::{Edge, Graph, Node};
pub use stats::GraphStats;
pub use storage::{load_graph, save_graph, StorageFormat};
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
