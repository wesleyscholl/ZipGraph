//! Node embeddings for graphs

use crate::error::{MlError, Result};
use ndarray::{Array1, Array2};
use rand::Rng;
use serde::{Deserialize, Serialize};
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

/// Node2Vec embeddings trainer (simplified version)
pub struct Node2VecTrainer {
    walk_length: usize,
    num_walks: usize,
    dimension: usize,
}

impl Node2VecTrainer {
    pub fn new(walk_length: usize, num_walks: usize, dimension: usize) -> Self {
        Self {
            walk_length,
            num_walks,
            dimension,
        }
    }

    /// Train Node2Vec embeddings on a graph
    pub fn train(&self, graph: &Graph) -> Result<NodeEmbeddings> {
        // Simplified implementation - in production, use proper random walks and Skip-gram
        let node_count = graph.node_count();
        let embeddings = NodeEmbeddings::new(node_count, self.dimension);

        // TODO: Implement proper Node2Vec algorithm
        // 1. Generate random walks
        // 2. Train Skip-gram model
        // 3. Extract embeddings

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
}
