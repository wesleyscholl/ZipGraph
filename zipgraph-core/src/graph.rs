//! Core graph data structures

use crate::error::{GraphError, Result};
use crate::types::{FeatureVector, NodeId, Weight};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Graph node with properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub properties: HashMap<String, f64>,
    pub features: Option<FeatureVector>,
}

impl Node {
    pub fn new(id: NodeId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            properties: HashMap::new(),
            features: None,
        }
    }

    pub fn with_property(mut self, key: impl Into<String>, value: f64) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    pub fn with_features(mut self, features: FeatureVector) -> Self {
        self.features = Some(features);
        self
    }
}

/// Graph edge with weight and type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: Weight,
    pub edge_type: String,
}

impl Edge {
    pub fn new(from: NodeId, to: NodeId, weight: Weight) -> Self {
        Self {
            from,
            to,
            weight,
            edge_type: "default".to_string(),
        }
    }

    pub fn with_type(mut self, edge_type: impl Into<String>) -> Self {
        self.edge_type = edge_type.into();
        self
    }
}

/// Main graph structure using adjacency list representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    /// Adjacency list: node_id -> Vec<(neighbor_id, edge_index)>
    adjacency: HashMap<NodeId, Vec<(NodeId, usize)>>,
    edges: Vec<Edge>,
    is_directed: bool,
    next_node_id: NodeId,
}

impl Graph {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self::with_capacity(0, 0)
    }

    /// Create a new graph with preallocated capacity
    pub fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            nodes: HashMap::with_capacity(node_capacity),
            adjacency: HashMap::with_capacity(node_capacity),
            edges: Vec::with_capacity(edge_capacity),
            is_directed: false,
            next_node_id: 0,
        }
    }

    /// Create a new directed graph
    pub fn new_directed() -> Self {
        let mut graph = Self::new();
        graph.is_directed = true;
        graph
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = node.id;
        self.nodes.insert(id, node);
        self.adjacency.entry(id).or_insert_with(Vec::new);
        if id >= self.next_node_id {
            self.next_node_id = id + 1;
        }
        id
    }

    /// Add a node with just a label
    pub fn add_node_simple(&mut self, label: impl Into<String>) -> NodeId {
        let id = self.next_node_id;
        self.add_node(Node::new(id, label))
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: Weight) -> Result<usize> {
        if !self.nodes.contains_key(&from) {
            return Err(GraphError::NodeNotFound(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(GraphError::NodeNotFound(to));
        }

        let edge_idx = self.edges.len();
        self.edges.push(Edge::new(from, to, weight));

        self.adjacency
            .get_mut(&from)
            .unwrap()
            .push((to, edge_idx));

        if !self.is_directed {
            self.adjacency
                .get_mut(&to)
                .unwrap()
                .push((from, edge_idx));
        }

        Ok(edge_idx)
    }

    /// Get a node by ID
    pub fn node(&self, id: NodeId) -> Result<&Node> {
        self.nodes
            .get(&id)
            .ok_or(GraphError::NodeNotFound(id))
    }

    /// Get neighbors of a node
    pub fn neighbors(&self, id: NodeId) -> Result<Vec<NodeId>> {
        self.adjacency
            .get(&id)
            .map(|neighbors| neighbors.iter().map(|(neighbor_id, _)| *neighbor_id).collect())
            .ok_or(GraphError::NodeNotFound(id))
    }

    /// Get all neighbors with weights
    pub fn neighbors_with_weights(&self, id: NodeId) -> Result<Vec<(NodeId, Weight)>> {
        self.adjacency
            .get(&id)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .map(|(neighbor_id, edge_idx)| {
                        (*neighbor_id, self.edges[*edge_idx].weight)
                    })
                    .collect()
            })
            .ok_or(GraphError::NodeNotFound(id))
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Check if graph is directed
    pub fn is_directed(&self) -> bool {
        self.is_directed
    }

    /// Get all node IDs
    pub fn node_ids(&self) -> Vec<NodeId> {
        self.nodes.keys().copied().collect()
    }

    /// Get all edges
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Calculate the degree of a node
    pub fn degree(&self, id: NodeId) -> Result<usize> {
        self.adjacency
            .get(&id)
            .map(|neighbors| neighbors.len())
            .ok_or(GraphError::NodeNotFound(id))
    }

    /// Clear all nodes and edges
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.adjacency.clear();
        self.edges.clear();
        self.next_node_id = 0;
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = Graph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_add_nodes_and_edges() {
        let mut graph = Graph::new();
        
        let n0 = graph.add_node_simple("A");
        let n1 = graph.add_node_simple("B");
        let n2 = graph.add_node_simple("C");

        graph.add_edge(n0, n1, 1.0).unwrap();
        graph.add_edge(n1, n2, 2.0).unwrap();

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_neighbors() {
        let mut graph = Graph::new();
        
        let n0 = graph.add_node_simple("A");
        let n1 = graph.add_node_simple("B");
        let n2 = graph.add_node_simple("C");

        graph.add_edge(n0, n1, 1.0).unwrap();
        graph.add_edge(n0, n2, 2.0).unwrap();

        let neighbors = graph.neighbors(n0).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&n1));
        assert!(neighbors.contains(&n2));
    }
}
