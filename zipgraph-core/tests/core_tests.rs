use zipgraph_core::{Graph, GraphStats};

#[test]
fn test_graph_creation() {
    let graph = Graph::new();
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_graph_directed() {
    let graph = Graph::new_directed();
    assert!(graph.is_directed());
}

#[test]
fn test_add_node_simple() {
    let mut graph = Graph::new();
    let id = graph.add_node_simple("Node A");
    assert_eq!(graph.node_count(), 1);
    assert_eq!(id, 0);
}

#[test]
fn test_add_multiple_nodes() {
    let mut graph = Graph::new();
    graph.add_node_simple("A");
    graph.add_node_simple("B");
    graph.add_node_simple("C");
    assert_eq!(graph.node_count(), 3);
}

#[test]
fn test_add_edge() {
    let mut graph = Graph::new();
    graph.add_node_simple("A");
    graph.add_node_simple("B");
    
    let result = graph.add_edge(0, 1, 1.0);
    assert!(result.is_ok());
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn test_add_edge_invalid_nodes() {
    let mut graph = Graph::new();
    graph.add_node_simple("A");
    
    let result = graph.add_edge(0, 999, 1.0);
    assert!(result.is_err());
}

#[test]
fn test_graph_capacity() {
    let graph = Graph::with_capacity(100, 200);
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_get_neighbors() {
    let mut graph = Graph::new();
    graph.add_node_simple("A");
    graph.add_node_simple("B");
    graph.add_node_simple("C");
    graph.add_edge(0, 1, 1.0).unwrap();
    graph.add_edge(0, 2, 1.0).unwrap();
    
    let neighbors = graph.neighbors(0);
    assert!(neighbors.is_ok());
    assert_eq!(neighbors.unwrap().len(), 2);
}

#[test]
fn test_node_degree() {
    let mut graph = Graph::new();
    graph.add_node_simple("A");
    graph.add_node_simple("B");
    graph.add_node_simple("C");
    graph.add_edge(0, 1, 1.0).unwrap();
    graph.add_edge(0, 2, 1.0).unwrap();
    
    let degree = graph.degree(0);
    assert!(degree.is_ok());
    assert_eq!(degree.unwrap(), 2);
}

#[test]
fn test_clear_graph() {
    let mut graph = Graph::new();
    graph.add_node_simple("A");
    graph.add_node_simple("B");
    graph.add_edge(0, 1, 1.0).unwrap();
    
    graph.clear();
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_triangle_graph() {
    let mut graph = Graph::new();
    graph.add_node_simple("A");
    graph.add_node_simple("B");
    graph.add_node_simple("C");
    graph.add_edge(0, 1, 1.0).unwrap();
    graph.add_edge(1, 2, 1.0).unwrap();
    graph.add_edge(2, 0, 1.0).unwrap();
    
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 3);
    assert_eq!(graph.degree(0).unwrap(), 2);
    assert_eq!(graph.degree(1).unwrap(), 2);
    assert_eq!(graph.degree(2).unwrap(), 2);
}

#[test]
fn test_star_graph() {
    let mut graph = Graph::new();
    let center = graph.add_node_simple("Center");
    
    for i in 0..5 {
        let node = graph.add_node_simple(format!("Leaf{}", i));
        graph.add_edge(center, node, 1.0).unwrap();
    }
    
    assert_eq!(graph.node_count(), 6);
    assert_eq!(graph.edge_count(), 5);
    assert_eq!(graph.degree(center).unwrap(), 5);
}

#[test]
fn test_large_graph_creation() {
    let mut graph = Graph::with_capacity(1000, 5000);
    
    for i in 0..1000 {
        graph.add_node_simple(format!("Node{}", i));
    }
    
    assert_eq!(graph.node_count(), 1000);
}

#[test]
fn test_neighbors_with_weights() {
    let mut graph = Graph::new();
    graph.add_node_simple("A");
    graph.add_node_simple("B");
    graph.add_node_simple("C");
    graph.add_edge(0, 1, 2.5).unwrap();
    graph.add_edge(0, 2, 3.5).unwrap();
    
    let neighbors = graph.neighbors_with_weights(0);
    assert!(neighbors.is_ok());
    assert_eq!(neighbors.unwrap().len(), 2);
}

#[test]
fn test_path_graph() {
    let mut graph = Graph::new();
    for i in 0..10 {
        graph.add_node_simple(format!("Node{}", i));
    }
    for i in 0..9 {
        graph.add_edge(i, i + 1, 1.0).unwrap();
    }
    
    assert_eq!(graph.node_count(), 10);
    assert_eq!(graph.edge_count(), 9);
    assert_eq!(graph.degree(0).unwrap(), 1);
    assert_eq!(graph.degree(9).unwrap(), 1);
    assert_eq!(graph.degree(5).unwrap(), 2);
}

#[test]
fn test_complete_graph() {
    let mut graph = Graph::new();
    for i in 0..4 {
        graph.add_node_simple(format!("Node{}", i));
    }
    
    for i in 0..4 {
        for j in (i+1)..4 {
            graph.add_edge(i, j, 1.0).unwrap();
        }
    }
    
    assert_eq!(graph.node_count(), 4);
    assert_eq!(graph.edge_count(), 6);
    
    for i in 0..4 {
        assert_eq!(graph.degree(i).unwrap(), 3);
    }
}

#[test]
fn test_graph_stats() {
    let mut graph = Graph::new();
    for i in 0..10 {
        graph.add_node_simple(format!("Node{}", i));
    }
    for i in 0..9 {
        graph.add_edge(i, i + 1, 1.0).unwrap();
    }
    
    let stats = GraphStats::from_graph(&graph);
    assert_eq!(stats.node_count, 10);
    assert_eq!(stats.edge_count, 9);
    assert!(stats.avg_degree > 0.0);
}

#[test]
fn test_empty_graph_stats() {
    let graph = Graph::new();
    let stats = GraphStats::from_graph(&graph);
    
    assert_eq!(stats.node_count, 0);
    assert_eq!(stats.edge_count, 0);
}

#[test]
fn test_stats_star_graph() {
    let mut graph = Graph::new();
    let center = graph.add_node_simple("Center");
    
    for i in 0..10 {
        let node = graph.add_node_simple(format!("Leaf{}", i));
        graph.add_edge(center, node, 1.0).unwrap();
    }
    
    let stats = GraphStats::from_graph(&graph);
    assert_eq!(stats.node_count, 11);
    assert_eq!(stats.edge_count, 10);
    assert!(stats.avg_degree > 1.0);
    assert!(stats.avg_degree < 3.0);
}
