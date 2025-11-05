//! Feature extraction from graphs for ML models

use zipgraph_core::{Graph, GraphStats};

/// Extract features from a graph for ML models
pub struct FeatureExtractor;

impl FeatureExtractor {
    /// Extract basic statistical features
    pub fn extract_basic_features(graph: &Graph) -> Vec<f64> {
        let stats = GraphStats::from_graph(graph);
        stats.to_feature_vector()
    }

    /// Extract node-level features
    pub fn extract_node_features(graph: &Graph, node_id: usize) -> Vec<f64> {
        let mut features = Vec::new();

        // Degree
        if let Ok(degree) = graph.degree(node_id) {
            features.push(degree as f64);
        } else {
            features.push(0.0);
        }

        // Neighbor count
        if let Ok(neighbors) = graph.neighbors(node_id) {
            features.push(neighbors.len() as f64);
        } else {
            features.push(0.0);
        }

        features
    }

    /// Extract edge-level features
    pub fn extract_edge_features(graph: &Graph, from: usize, to: usize) -> Vec<f64> {
        let mut features = Vec::new();

        // Node degrees
        features.push(graph.degree(from).unwrap_or(0) as f64);
        features.push(graph.degree(to).unwrap_or(0) as f64);

        features
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_features() {
        let mut graph = Graph::new();
        let n0 = graph.add_node_simple("A");
        let n1 = graph.add_node_simple("B");
        graph.add_edge(n0, n1, 1.0).unwrap();

        let features = FeatureExtractor::extract_basic_features(&graph);
        assert!(!features.is_empty());
    }
}
