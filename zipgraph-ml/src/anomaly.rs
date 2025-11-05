//! Anomaly detection in graphs

use crate::embeddings::NodeEmbeddings;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use zipgraph_core::{Graph, NodeId};

/// Detected anomaly in a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub node_ids: Vec<NodeId>,
    pub anomaly_score: f64,
    pub reason: String,
    pub anomaly_type: AnomalyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    StructuralAnomaly,
    DegreeAnomaly,
    EmbeddingAnomaly,
    PatternAnomaly,
}

/// Anomaly detector using ML techniques
pub struct AnomalyDetector {
    baseline_embeddings: Option<NodeEmbeddings>,
    anomaly_threshold: f64,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new() -> Self {
        Self {
            baseline_embeddings: None,
            anomaly_threshold: 0.8,
        }
    }

    /// Set the anomaly threshold (0.0 to 1.0)
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.anomaly_threshold = threshold;
        self
    }

    /// Train on baseline "normal" graph
    pub fn train_on_baseline(&mut self, graph: &Graph) -> Result<()> {
        // Generate embeddings for baseline
        let embeddings = NodeEmbeddings::new(graph.node_count(), 64);
        self.baseline_embeddings = Some(embeddings);
        Ok(())
    }

    /// Detect anomalies in a graph
    pub fn detect(&self, graph: &Graph) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Detect degree anomalies
        anomalies.extend(self.detect_degree_anomalies(graph));

        // Detect structural anomalies
        anomalies.extend(self.detect_structural_anomalies(graph));

        anomalies
    }

    /// Detect nodes with unusual degree
    fn detect_degree_anomalies(&self, graph: &Graph) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        
        let degrees: Vec<usize> = graph
            .node_ids()
            .iter()
            .filter_map(|&id| graph.degree(id).ok())
            .collect();

        if degrees.is_empty() {
            return anomalies;
        }

        let mean_degree: f64 = degrees.iter().sum::<usize>() as f64 / degrees.len() as f64;
        let variance: f64 = degrees
            .iter()
            .map(|&d| {
                let diff = d as f64 - mean_degree;
                diff * diff
            })
            .sum::<f64>()
            / degrees.len() as f64;
        let std_dev = variance.sqrt();

        for &node_id in graph.node_ids().iter() {
            if let Ok(degree) = graph.degree(node_id) {
                let z_score = ((degree as f64 - mean_degree) / std_dev).abs();
                
                if z_score > 3.0 {
                    // Degree is more than 3 standard deviations from mean
                    anomalies.push(Anomaly {
                        node_ids: vec![node_id],
                        anomaly_score: z_score / 10.0,
                        reason: format!("Node {} has unusual degree: {}", node_id, degree),
                        anomaly_type: AnomalyType::DegreeAnomaly,
                    });
                }
            }
        }

        anomalies
    }

    /// Detect structural anomalies
    fn detect_structural_anomalies(&self, graph: &Graph) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Detect isolated nodes (degree 0)
        for &node_id in graph.node_ids().iter() {
            if let Ok(degree) = graph.degree(node_id) {
                if degree == 0 {
                    anomalies.push(Anomaly {
                        node_ids: vec![node_id],
                        anomaly_score: 1.0,
                        reason: format!("Node {} is isolated", node_id),
                        anomaly_type: AnomalyType::StructuralAnomaly,
                    });
                }
            }
        }

        anomalies
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = AnomalyDetector::new();
        assert_eq!(detector.anomaly_threshold, 0.8);
    }

    #[test]
    fn test_degree_anomaly_detection() {
        let mut graph = Graph::new();
        
        // Create normal nodes
        for i in 0..10 {
            graph.add_node_simple(format!("Node{}", i));
        }
        
        // Add normal edges
        for i in 0..9 {
            graph.add_edge(i, i + 1, 1.0).unwrap();
        }
        
        // Add anomalous hub node
        let hub = graph.add_node_simple("Hub");
        for i in 0..10 {
            graph.add_edge(hub, i, 1.0).unwrap();
        }

        let detector = AnomalyDetector::new();
        let anomalies = detector.detect(&graph);
        
        // Should detect the hub as anomalous
        assert!(!anomalies.is_empty());
    }
}
