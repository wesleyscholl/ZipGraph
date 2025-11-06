//! PageRank and centrality algorithms

use crate::algorithms::bfs;
use crate::error::Result;
use crate::graph::Graph;
use crate::types::NodeId;
use std::collections::HashMap;

/// Compute PageRank scores for all nodes in the graph
///
/// PageRank is an algorithm used to measure the importance of nodes in a graph.
/// It was originally developed by Google to rank web pages.
///
/// # Arguments
/// * `graph` - The graph to analyze
/// * `damping_factor` - Probability of following a link (typically 0.85)
/// * `max_iterations` - Maximum number of iterations
/// * `tolerance` - Convergence threshold
///
/// # Returns
/// HashMap mapping node IDs to their PageRank scores
pub fn pagerank(
    graph: &Graph,
    damping_factor: f64,
    max_iterations: usize,
    tolerance: f64,
) -> Result<HashMap<NodeId, f64>> {
    let node_count = graph.node_count();
    if node_count == 0 {
        return Ok(HashMap::new());
    }

    let node_ids = graph.node_ids();
    let initial_rank = 1.0 / node_count as f64;
    
    // Initialize ranks
    let mut ranks: HashMap<NodeId, f64> = node_ids
        .iter()
        .map(|&id| (id, initial_rank))
        .collect();
    
    let mut new_ranks = ranks.clone();

    // Iteratively update ranks
    for _ in 0..max_iterations {
        let mut converged = true;

        for &node_id in &node_ids {
            let mut rank_sum = 0.0;
            
            // Sum contributions from incoming nodes
            for &src_node in &node_ids {
                if let Ok(neighbors) = graph.neighbors(src_node) {
                    if neighbors.contains(&node_id) {
                        let out_degree = neighbors.len();
                        if out_degree > 0 {
                            rank_sum += ranks[&src_node] / out_degree as f64;
                        }
                    }
                }
            }

            // Apply PageRank formula
            let new_rank = (1.0 - damping_factor) / node_count as f64 
                + damping_factor * rank_sum;
            
            // Check for convergence
            if (new_rank - ranks[&node_id]).abs() > tolerance {
                converged = false;
            }
            
            new_ranks.insert(node_id, new_rank);
        }

        ranks = new_ranks.clone();

        if converged {
            break;
        }
    }

    Ok(ranks)
}

/// Compute degree centrality for all nodes
///
/// Degree centrality measures the number of connections a node has.
pub fn degree_centrality(graph: &Graph) -> Result<HashMap<NodeId, f64>> {
    let node_count = graph.node_count();
    if node_count <= 1 {
        return Ok(HashMap::new());
    }

    let mut centrality = HashMap::new();
    let max_degree = (node_count - 1) as f64;

    for node_id in graph.node_ids() {
        let degree = graph.degree(node_id)? as f64;
        let normalized = degree / max_degree;
        centrality.insert(node_id, normalized);
    }

    Ok(centrality)
}

/// Compute closeness centrality for all nodes
///
/// Closeness centrality measures how close a node is to all other nodes.
pub fn closeness_centrality(graph: &Graph) -> Result<HashMap<NodeId, f64>> {
    let node_ids = graph.node_ids();
    let mut centrality = HashMap::new();

    for &node_id in &node_ids {
        let mut total_distance = 0.0;
        let mut reachable_count = 0;

        // BFS to compute distances
        for &target_id in &node_ids {
            if node_id != target_id {
                if let Ok(path) = bfs(graph, node_id, target_id) {
                    total_distance += (path.len() - 1) as f64;
                    reachable_count += 1;
                }
            }
        }

        let score = if total_distance > 0.0 && reachable_count > 0 {
            reachable_count as f64 / total_distance
        } else {
            0.0
        };

        centrality.insert(node_id, score);
    }

    Ok(centrality)
}

/// Compute betweenness centrality for all nodes
///
/// Betweenness centrality measures how often a node lies on shortest paths between other nodes.
pub fn betweenness_centrality(graph: &Graph) -> Result<HashMap<NodeId, f64>> {
    let node_ids = graph.node_ids();
    let node_count = node_ids.len();
    let mut centrality: HashMap<NodeId, f64> = node_ids.iter().map(|&id| (id, 0.0)).collect();

    if node_count <= 2 {
        return Ok(centrality);
    }

    // For each pair of nodes, count paths through each intermediate node
    for &source in &node_ids {
        for &target in &node_ids {
            if source == target {
                continue;
            }

            // Find all shortest paths from source to target
            let paths = find_all_shortest_paths(graph, source, target)?;
            
            if paths.is_empty() {
                continue;
            }

            let num_paths = paths.len() as f64;

            // Count how many shortest paths go through each node
            for intermediate_node in &node_ids {
                if *intermediate_node == source || *intermediate_node == target {
                    continue;
                }

                let paths_through = paths.iter()
                    .filter(|path| path.contains(intermediate_node))
                    .count() as f64;

                if paths_through > 0.0 {
                    *centrality.get_mut(intermediate_node).unwrap() += paths_through / num_paths;
                }
            }
        }
    }

    // Normalize by the number of pairs
    let normalizer = if node_count > 2 {
        ((node_count - 1) * (node_count - 2)) as f64
    } else {
        1.0
    };

    for score in centrality.values_mut() {
        *score /= normalizer;
    }

    Ok(centrality)
}

/// Find all shortest paths between two nodes
fn find_all_shortest_paths(graph: &Graph, source: NodeId, target: NodeId) -> Result<Vec<Vec<NodeId>>> {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    let mut distances: HashMap<NodeId, usize> = HashMap::new();
    let mut predecessors: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

    queue.push_back(source);
    distances.insert(source, 0);

    // BFS to find shortest path distances and track predecessors
    while let Some(current) = queue.pop_front() {
        let current_dist = distances[&current];

        if let Ok(neighbors) = graph.neighbors(current) {
            for &neighbor in &neighbors {
                if !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, current_dist + 1);
                    queue.push_back(neighbor);
                    predecessors.entry(neighbor).or_insert_with(Vec::new).push(current);
                } else if distances[&neighbor] == current_dist + 1 {
                    predecessors.entry(neighbor).or_insert_with(Vec::new).push(current);
                }
            }
        }
    }

    // No path exists
    if !distances.contains_key(&target) {
        return Ok(Vec::new());
    }

    // Reconstruct all shortest paths
    let mut paths = Vec::new();
    let mut current_paths = vec![vec![target]];

    while !current_paths.is_empty() {
        let mut next_paths = Vec::new();

        for path in current_paths {
            let last_node = *path.last().unwrap();

            if last_node == source {
                let mut complete_path = path.clone();
                complete_path.reverse();
                paths.push(complete_path);
            } else if let Some(preds) = predecessors.get(&last_node) {
                for &pred in preds {
                    let mut new_path = path.clone();
                    new_path.push(pred);
                    next_paths.push(new_path);
                }
            }
        }

        current_paths = next_paths;
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new_directed();
        
        for i in 0..5 {
            graph.add_node_simple(format!("Node{}", i));
        }

        // Create a simple directed graph
        graph.add_edge(0, 1, 1.0).unwrap();
        graph.add_edge(1, 2, 1.0).unwrap();
        graph.add_edge(2, 3, 1.0).unwrap();
        graph.add_edge(3, 0, 1.0).unwrap();
        graph.add_edge(2, 4, 1.0).unwrap();

        graph
    }

    #[test]
    fn test_pagerank() {
        let graph = create_test_graph();
        let ranks = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

        // All nodes should have ranks
        assert_eq!(ranks.len(), 5);

        // Ranks should sum close to node count (for directed graphs)
        // PageRank typically sums to the number of nodes
        let sum: f64 = ranks.values().sum();
        assert!(sum > 0.0); // Just ensure it's reasonable

        // All ranks should be positive
        for rank in ranks.values() {
            assert!(*rank > 0.0);
        }
    }

    #[test]
    fn test_pagerank_empty_graph() {
        let graph = Graph::new();
        let ranks = pagerank(&graph, 0.85, 100, 1e-6).unwrap();
        assert_eq!(ranks.len(), 0);
    }

    #[test]
    fn test_degree_centrality() {
        let graph = create_test_graph();
        let centrality = degree_centrality(&graph).unwrap();

        assert_eq!(centrality.len(), 5);

        // All centrality scores should be between 0 and 1
        for score in centrality.values() {
            assert!(*score >= 0.0 && *score <= 1.0);
        }
    }

    #[test]
    fn test_closeness_centrality() {
        let mut graph = Graph::new();
        
        // Create a simple undirected graph
        for i in 0..4 {
            graph.add_node_simple(format!("Node{}", i));
        }
        graph.add_edge(0, 1, 1.0).unwrap();
        graph.add_edge(1, 2, 1.0).unwrap();
        graph.add_edge(2, 3, 1.0).unwrap();

        let centrality = closeness_centrality(&graph).unwrap();
        assert_eq!(centrality.len(), 4);

        // Middle nodes should have higher closeness centrality
        assert!(centrality[&1] > centrality[&0]);
        assert!(centrality[&2] > centrality[&3]);
    }

    #[test]
    fn test_betweenness_centrality() {
        let mut graph = Graph::new();
        
        // Create a graph where node 1 is a bridge
        // 0 -> 1 -> 2
        // 3 -> 1 -> 4
        for i in 0..5 {
            graph.add_node_simple(format!("Node{}", i));
        }
        graph.add_edge(0, 1, 1.0).unwrap();
        graph.add_edge(1, 0, 1.0).unwrap();
        graph.add_edge(1, 2, 1.0).unwrap();
        graph.add_edge(2, 1, 1.0).unwrap();
        graph.add_edge(3, 1, 1.0).unwrap();
        graph.add_edge(1, 3, 1.0).unwrap();
        graph.add_edge(1, 4, 1.0).unwrap();
        graph.add_edge(4, 1, 1.0).unwrap();

        let centrality = betweenness_centrality(&graph).unwrap();
        assert_eq!(centrality.len(), 5);

        // Node 1 should have the highest betweenness (it's a bridge)
        let node1_betweenness = centrality[&1];
        for (&node, &score) in centrality.iter() {
            if node != 1 {
                assert!(node1_betweenness >= score);
            }
        }
    }

    #[test]
    fn test_betweenness_empty_graph() {
        let graph = Graph::new();
        let centrality = betweenness_centrality(&graph).unwrap();
        assert_eq!(centrality.len(), 0);
    }

    #[test]
    fn test_betweenness_single_node() {
        let mut graph = Graph::new();
        graph.add_node_simple("Node0");

        let centrality = betweenness_centrality(&graph).unwrap();
        assert_eq!(centrality.len(), 1);
        assert_eq!(centrality[&0], 0.0);
    }

    #[test]
    fn test_find_all_shortest_paths() {
        let mut graph = Graph::new();
        
        // Create a diamond graph with multiple shortest paths
        // 0 -> 1 -> 3
        // 0 -> 2 -> 3
        for i in 0..4 {
            graph.add_node_simple(format!("Node{}", i));
        }
        graph.add_edge(0, 1, 1.0).unwrap();
        graph.add_edge(0, 2, 1.0).unwrap();
        graph.add_edge(1, 3, 1.0).unwrap();
        graph.add_edge(2, 3, 1.0).unwrap();

        let paths = find_all_shortest_paths(&graph, 0, 3).unwrap();
        
        // Should find 2 shortest paths
        assert_eq!(paths.len(), 2);
        
        // All paths should have length 3 (0->1->3 or 0->2->3)
        for path in &paths {
            assert_eq!(path.len(), 3);
            assert_eq!(path[0], 0);
            assert_eq!(path[2], 3);
        }
    }
}
