//! Node embeddings for graphs

use crate::error::{MlError, Result};
use ndarray::{Array1, Array2};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zipgraph_core::{Graph, NodeId};

/// Node embeddings representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEmbeddings {
    embeddings: Array2<f32>,
    dimension: usize,
    node_count: usize,
}

impl NodeEmbeddings {
    /// Create new embeddings with random initialization
    pub fn new(node_count: usize, dimension: usize) -> Self {
        let mut rng = rand::thread_rng();
        let embeddings = Array2::from_shape_fn((node_count, dimension), |_| {
            rng.gen_range(-0.1..0.1)
        });

        Self {
            embeddings,
            dimension,
            node_count,
        }
    }

    /// Get embedding for a node
    pub fn get_embedding(&self, node_id: NodeId) -> Result<Array1<f32>> {
        if node_id >= self.node_count {
            return Err(MlError::FeatureError(format!(
                "Node {} out of bounds",
                node_id
            )));
        }
        Ok(self.embeddings.row(node_id).to_owned())
    }

    /// Set embedding for a node
    pub fn set_embedding(&mut self, node_id: NodeId, embedding: &Array1<f32>) -> Result<()> {
        if node_id >= self.node_count {
            return Err(MlError::FeatureError(format!(
                "Node {} out of bounds",
                node_id
            )));
        }
        if embedding.len() != self.dimension {
            return Err(MlError::FeatureError(format!(
                "Embedding dimension mismatch: expected {}, got {}",
                self.dimension,
                embedding.len()
            )));
        }
        self.embeddings.row_mut(node_id).assign(embedding);
        Ok(())
    }

    /// Calculate cosine similarity between two nodes
    pub fn cosine_similarity(&self, node_a: NodeId, node_b: NodeId) -> Result<f32> {
        let emb_a = self.get_embedding(node_a)?;
        let emb_b = self.get_embedding(node_b)?;

        let dot_product: f32 = emb_a.iter().zip(emb_b.iter()).map(|(a, b)| a * b).sum();
        let norm_a: f32 = emb_a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = emb_b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(0.0);
        }

        Ok(dot_product / (norm_a * norm_b))
    }

    /// Get dimension of embeddings
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.node_count
    }
}

/// Node2Vec embeddings trainer
pub struct Node2VecTrainer {
    walk_length: usize,
    num_walks: usize,
    dimension: usize,
    p: f64, // Return parameter
    q: f64, // In-out parameter
}

impl Node2VecTrainer {
    pub fn new(walk_length: usize, num_walks: usize, dimension: usize) -> Self {
        Self {
            walk_length,
            num_walks,
            dimension,
            p: 1.0,
            q: 1.0,
        }
    }

    /// Set the return and in-out parameters for biased random walks
    pub fn with_params(mut self, p: f64, q: f64) -> Self {
        self.p = p;
        self.q = q;
        self
    }

    /// Generate a single random walk starting from a node
    fn random_walk(&self, graph: &Graph, start_node: NodeId) -> Vec<NodeId> {
        let mut walk = vec![start_node];
        let mut rng = rand::thread_rng();

        for _ in 1..self.walk_length {
            let current = *walk.last().unwrap();
            
            match graph.neighbors(current) {
                Ok(neighbors) if !neighbors.is_empty() => {
                    // Simple random selection (can be enhanced with biased sampling)
                    let idx = rng.gen_range(0..neighbors.len());
                    walk.push(neighbors[idx]);
                }
                _ => break,
            }
        }

        walk
    }

    /// Generate all random walks for the graph
    pub fn generate_walks(&self, graph: &Graph) -> Vec<Vec<NodeId>> {
        let mut all_walks = Vec::new();
        let node_ids = graph.node_ids();

        for _ in 0..self.num_walks {
            for &node_id in &node_ids {
                let walk = self.random_walk(graph, node_id);
                if walk.len() > 1 {
                    all_walks.push(walk);
                }
            }
        }

        all_walks
    }

    /// Train Node2Vec embeddings on a graph
    pub fn train(&self, graph: &Graph) -> Result<NodeEmbeddings> {
        let node_count = graph.node_count();
        if node_count == 0 {
            return Err(MlError::TrainingError("Empty graph".to_string()));
        }

        // Generate random walks
        let walks = self.generate_walks(graph);
        
        if walks.is_empty() {
            return Err(MlError::TrainingError("No walks generated".to_string()));
        }

        // Initialize embeddings with small random values
        let mut embeddings = NodeEmbeddings::new(node_count, self.dimension);
        
        // Simple embedding update based on co-occurrence in walks
        // In a full implementation, this would be Skip-gram with negative sampling
        let mut co_occurrence: HashMap<(NodeId, NodeId), usize> = HashMap::new();
        
        for walk in &walks {
            for i in 0..walk.len() {
                for j in (i + 1)..(walk.len()).min(i + 5) {
                    let key = (walk[i].min(walk[j]), walk[i].max(walk[j]));
                    *co_occurrence.entry(key).or_insert(0) += 1;
                }
            }
        }

        // Update embeddings based on co-occurrence (simplified)
        for ((node_a, node_b), count) in co_occurrence.iter() {
            if *count > 5 {
                // Nodes that co-occur frequently should have similar embeddings
                let weight = (*count as f32).log2() * 0.01;
                
                if let (Ok(emb_a), Ok(emb_b)) = (
                    embeddings.get_embedding(*node_a),
                    embeddings.get_embedding(*node_b)
                ) {
                    let mut updated_a = emb_a.clone();
                    let mut updated_b = emb_b.clone();
                    
                    for i in 0..self.dimension {
                        updated_a[i] += (updated_b[i] - updated_a[i]) * weight;
                        updated_b[i] += (updated_a[i] - updated_b[i]) * weight;
                    }
                    
                    let _ = embeddings.set_embedding(*node_a, &updated_a);
                    let _ = embeddings.set_embedding(*node_b, &updated_b);
                }
            }
        }

        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embeddings_creation() {
        let embeddings = NodeEmbeddings::new(10, 64);
        assert_eq!(embeddings.node_count(), 10);
        assert_eq!(embeddings.dimension(), 64);
    }

    #[test]
    fn test_get_embedding() {
        let embeddings = NodeEmbeddings::new(10, 64);
        let emb = embeddings.get_embedding(5).unwrap();
        assert_eq!(emb.len(), 64);
    }

    #[test]
    fn test_cosine_similarity() {
        let embeddings = NodeEmbeddings::new(10, 64);
        let sim = embeddings.cosine_similarity(0, 1).unwrap();
        assert!(sim >= -1.0 && sim <= 1.0);
    }

    #[test]
    fn test_node2vec_random_walk() {
        let mut graph = Graph::new();
        let n0 = graph.add_node_simple("0");
        let n1 = graph.add_node_simple("1");
        let n2 = graph.add_node_simple("2");
        let n3 = graph.add_node_simple("3");
        
        graph.add_edge(n0, n1, 1.0).unwrap();
        graph.add_edge(n1, n2, 1.0).unwrap();
        graph.add_edge(n2, n3, 1.0).unwrap();
        graph.add_edge(n3, n0, 1.0).unwrap();

        let trainer = Node2VecTrainer::new(10, 5, 16);
        let walk = trainer.random_walk(&graph, n0);
        
        assert!(!walk.is_empty());
        assert_eq!(walk[0], n0);
        assert!(walk.len() <= 10);
    }

    #[test]
    fn test_node2vec_generate_walks() {
        let mut graph = Graph::new();
        let n0 = graph.add_node_simple("0");
        let n1 = graph.add_node_simple("1");
        let n2 = graph.add_node_simple("2");
        
        graph.add_edge(n0, n1, 1.0).unwrap();
        graph.add_edge(n1, n2, 1.0).unwrap();

        let trainer = Node2VecTrainer::new(5, 3, 16);
        let walks = trainer.generate_walks(&graph);
        
        assert!(!walks.is_empty());
        // Should generate walks for each node × num_walks
        assert!(walks.len() >= 3); // At least some walks succeed
    }

    #[test]
    fn test_node2vec_train() {
        let mut graph = Graph::new();
        let n0 = graph.add_node_simple("0");
        let n1 = graph.add_node_simple("1");
        let n2 = graph.add_node_simple("2");
        let n3 = graph.add_node_simple("3");
        
        graph.add_edge(n0, n1, 1.0).unwrap();
        graph.add_edge(n1, n2, 1.0).unwrap();
        graph.add_edge(n2, n3, 1.0).unwrap();
        graph.add_edge(n3, n0, 1.0).unwrap();

        let trainer = Node2VecTrainer::new(10, 5, 16);
        let embeddings = trainer.train(&graph).unwrap();
        
        assert_eq!(embeddings.node_count(), 4);
        assert_eq!(embeddings.dimension(), 16);
    }

    #[test]
    fn test_node2vec_with_params() {
        let trainer = Node2VecTrainer::new(10, 5, 16)
            .with_params(2.0, 0.5);
        
        let mut graph = Graph::new();
        let n0 = graph.add_node_simple("0");
        let n1 = graph.add_node_simple("1");
        graph.add_edge(n0, n1, 1.0).unwrap();
        
        let result = trainer.train(&graph);
        assert!(result.is_ok());
    }

    #[test]
    fn test_node2vec_empty_graph() {
        let graph = Graph::new();
        let trainer = Node2VecTrainer::new(10, 5, 16);
        
        let result = trainer.train(&graph);
        assert!(result.is_err());
    }
}
