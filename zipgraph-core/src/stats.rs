//! Graph statistics and analysis

use crate::graph::Graph;
use serde::{Deserialize, Serialize};

/// Graph statistics used for ML features
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub avg_degree: f64,
    pub max_degree: usize,
    pub min_degree: usize,
    pub density: f64,
    pub is_directed: bool,
    pub clustering_coefficient: Option<f64>,
    pub diameter: Option<usize>,
}

impl GraphStats {
    /// Calculate statistics for a graph
    pub fn from_graph(graph: &Graph) -> Self {
        let node_count = graph.node_count();
        let edge_count = graph.edge_count();
        
        if node_count == 0 {
            return Self::default();
        }

        let degrees: Vec<usize> = graph
            .node_ids()
            .iter()
            .filter_map(|&id| graph.degree(id).ok())
            .collect();

        let total_degree: usize = degrees.iter().sum();
        let avg_degree = total_degree as f64 / node_count as f64;
        let max_degree = *degrees.iter().max().unwrap_or(&0);
        let min_degree = *degrees.iter().min().unwrap_or(&0);

        let max_edges = if graph.is_directed() {
            node_count * (node_count - 1)
        } else {
            node_count * (node_count - 1) / 2
        };

        let density = if max_edges > 0 {
            edge_count as f64 / max_edges as f64
        } else {
            0.0
        };

        Self {
            node_count,
            edge_count,
            avg_degree,
            max_degree,
            min_degree,
            density,
            is_directed: graph.is_directed(),
            clustering_coefficient: None, // Computed on demand
            diameter: None,               // Computed on demand
        }
    }

    /// Check if the graph is sparse
    pub fn is_sparse(&self) -> bool {
        self.density < 0.1
    }

    /// Check if the graph is dense
    pub fn is_dense(&self) -> bool {
        self.density > 0.5
    }

    /// Get feature vector for ML algorithms
    pub fn to_feature_vector(&self) -> Vec<f64> {
        vec![
            self.node_count as f64,
            self.edge_count as f64,
            self.avg_degree,
            self.max_degree as f64,
            self.min_degree as f64,
            self.density,
            if self.is_directed { 1.0 } else { 0.0 },
            self.clustering_coefficient.unwrap_or(0.0),
            self.diameter.unwrap_or(0) as f64,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_empty_graph() {
        let graph = Graph::new();
        let stats = GraphStats::from_graph(&graph);
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
    }

    #[test]
    fn test_stats_simple_graph() {
        let mut graph = Graph::new();
        let n0 = graph.add_node_simple("A");
        let n1 = graph.add_node_simple("B");
        graph.add_edge(n0, n1, 1.0).unwrap();

        let stats = GraphStats::from_graph(&graph);
        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.edge_count, 1);
        assert!(stats.avg_degree > 0.0);
    }
}
