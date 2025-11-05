//! Query representation and execution

use serde::{Deserialize, Serialize};
use zipgraph_core::NodeId;

/// Query types supported by the optimizer
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Query {
    ShortestPath {
        start: NodeId,
        goal: NodeId,
    },
    Neighbors {
        node: NodeId,
    },
    PageRank {
        iterations: usize,
    },
    ConnectedComponents,
    ShortestPaths {
        start: NodeId,
    },
}

/// Query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryResult {
    Path(Vec<NodeId>),
    Neighbors(Vec<NodeId>),
    Scores(Vec<(NodeId, f64)>),
    Components(Vec<Vec<NodeId>>),
}

impl Query {
    /// Generate a fingerprint for caching
    pub fn fingerprint(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
