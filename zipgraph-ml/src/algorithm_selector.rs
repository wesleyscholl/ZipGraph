//! ML-based algorithm selection

use crate::error::{MlError, Result};
use crate::features::FeatureExtractor;
use zipgraph_core::{Algorithm, Graph, GraphStats};

/// ML model for selecting the best algorithm for a graph
pub struct AlgorithmSelector {
    trained: bool,
}

impl AlgorithmSelector {
    /// Create a new algorithm selector
    pub fn new() -> Self {
        Self { trained: false }
    }

    /// Select the best algorithm based on graph properties
    pub fn select(&self, graph: &Graph) -> Algorithm {
        let stats = GraphStats::from_graph(graph);
        
        // Simple heuristic-based selection (replace with ML model in production)
        if stats.node_count < 100 {
            // Small graphs: BFS is fast enough
            Algorithm::BFS
        } else if stats.is_dense() {
            // Dense graphs: Floyd-Warshall for all-pairs shortest paths
            Algorithm::FloydWarshall
        } else if stats.is_sparse() {
            // Sparse graphs: Dijkstra is efficient
            Algorithm::Dijkstra
        } else {
            // Medium-density: A* with heuristics
            Algorithm::AStar
        }
    }

    /// Select algorithm for shortest path query
    pub fn select_shortest_path(&self, graph: &Graph, start: usize, goal: usize) -> Algorithm {
        let stats = GraphStats::from_graph(graph);
        
        // Heuristic: use BFS for unweighted, Dijkstra for weighted
        if stats.node_count < 1000 {
            Algorithm::Dijkstra
        } else {
            Algorithm::BidirectionalSearch
        }
    }

    /// Train the selector on historical data
    pub fn train(&mut self, training_data: Vec<(Graph, Algorithm, f64)>) -> Result<()> {
        // TODO: Implement actual ML training
        // 1. Extract features from graphs
        // 2. Train classification model (Random Forest, Neural Network)
        // 3. Validate on test set
        
        if training_data.is_empty() {
            return Err(MlError::TrainingError("No training data provided".to_string()));
        }

        self.trained = true;
        Ok(())
    }

    /// Check if the model is trained
    pub fn is_trained(&self) -> bool {
        self.trained
    }
}

impl Default for AlgorithmSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_creation() {
        let selector = AlgorithmSelector::new();
        assert!(!selector.is_trained());
    }

    #[test]
    fn test_algorithm_selection() {
        let selector = AlgorithmSelector::new();
        
        // Small graph
        let mut small_graph = Graph::new();
        for i in 0..10 {
            small_graph.add_node_simple(format!("Node{}", i));
        }
        
        let algo = selector.select(&small_graph);
        assert_eq!(algo, Algorithm::BFS);
    }

    #[test]
    fn test_shortest_path_selection() {
        let selector = AlgorithmSelector::new();
        let graph = Graph::new();
        
        let algo = selector.select_shortest_path(&graph, 0, 1);
        // Should select an appropriate algorithm
        assert!(matches!(algo, Algorithm::Dijkstra | Algorithm::BidirectionalSearch));
    }
}
