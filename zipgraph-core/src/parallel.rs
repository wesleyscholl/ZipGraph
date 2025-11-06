//! Parallel graph algorithms using Rayon

use crate::error::Result;
use crate::graph::Graph;
use crate::types::{NodeId, Weight};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

/// Parallel breadth-first search from multiple source nodes
///
/// Performs BFS from multiple sources simultaneously, useful for
/// finding shortest paths from a set of sources to a goal.
pub fn parallel_multi_source_bfs(
    graph: &Graph,
    sources: &[NodeId],
    goal: NodeId,
) -> Result<Option<(NodeId, Vec<NodeId>)>> {
    if sources.is_empty() {
        return Ok(None);
    }

    let result: Arc<Mutex<Option<(NodeId, Vec<NodeId>)>>> = Arc::new(Mutex::new(None));
    
    sources.par_iter().find_map_any(|&source| {
        match crate::algorithms::bfs(graph, source, goal) {
            Ok(path) => {
                let mut res = result.lock().unwrap();
                if res.is_none() || path.len() < res.as_ref().unwrap().1.len() {
                    *res = Some((source, path.clone()));
                }
                Some((source, path))
            }
            Err(_) => None,
        }
    });

    let final_result = result.lock().unwrap().clone();
    Ok(final_result)
}

/// Parallel single-source shortest paths
///
/// Computes shortest paths from a source to multiple destinations in parallel.
pub fn parallel_shortest_paths(
    graph: &Graph,
    source: NodeId,
    destinations: &[NodeId],
) -> Result<HashMap<NodeId, (Vec<NodeId>, Weight)>> {
    let results: Vec<_> = destinations
        .par_iter()
        .filter_map(|&dest| {
            crate::algorithms::dijkstra(graph, source, dest)
                .ok()
                .map(|(path, cost)| (dest, (path, cost)))
        })
        .collect();

    Ok(results.into_iter().collect())
}

/// Parallel node degree computation
///
/// Computes degrees for all nodes in parallel.
pub fn parallel_node_degrees(graph: &Graph) -> Result<HashMap<NodeId, usize>> {
    let node_ids = graph.node_ids();
    
    let degrees: Vec<_> = node_ids
        .par_iter()
        .filter_map(|&node_id| {
            graph.degree(node_id).ok().map(|degree| (node_id, degree))
        })
        .collect();

    Ok(degrees.into_iter().collect())
}

/// Parallel PageRank computation
///
/// Computes PageRank using parallel operations for better performance
/// on large graphs.
pub fn parallel_pagerank(
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
    
    let mut ranks: HashMap<NodeId, f64> = node_ids
        .iter()
        .map(|&id| (id, initial_rank))
        .collect();

    for _ in 0..max_iterations {
        let new_ranks: HashMap<NodeId, f64> = node_ids
            .par_iter()
            .map(|&node_id| {
                let mut rank_sum = 0.0;
                
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

                let new_rank = (1.0 - damping_factor) / node_count as f64 
                    + damping_factor * rank_sum;
                
                (node_id, new_rank)
            })
            .collect();

        // Check convergence
        let converged = ranks.iter().all(|(id, &old_rank)| {
            (new_ranks[id] - old_rank).abs() <= tolerance
        });

        ranks = new_ranks;

        if converged {
            break;
        }
    }

    Ok(ranks)
}

/// Parallel neighborhood search
///
/// Find all neighbors within k hops for multiple nodes in parallel.
pub fn parallel_k_hop_neighbors(
    graph: &Graph,
    sources: &[NodeId],
    k: usize,
) -> Result<HashMap<NodeId, HashSet<NodeId>>> {
    let results: Vec<_> = sources
        .par_iter()
        .filter_map(|&source| {
            k_hop_neighbors(graph, source, k)
                .ok()
                .map(|neighbors| (source, neighbors))
        })
        .collect();

    Ok(results.into_iter().collect())
}

/// Helper function to find k-hop neighbors
fn k_hop_neighbors(graph: &Graph, source: NodeId, k: usize) -> Result<HashSet<NodeId>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back((source, 0));
    visited.insert(source);

    while let Some((node, depth)) = queue.pop_front() {
        if depth < k {
            if let Ok(neighbors) = graph.neighbors(node) {
                for neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back((neighbor, depth + 1));
                    }
                }
            }
        }
    }

    visited.remove(&source); // Remove source itself
    Ok(visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();
        
        for i in 0..10 {
            graph.add_node_simple(format!("Node{}", i));
        }

        for i in 0..9 {
            graph.add_edge(i, i + 1, 1.0).unwrap();
        }

        graph
    }

    #[test]
    fn test_parallel_multi_source_bfs() {
        let graph = create_test_graph();
        let sources = vec![0, 1, 2];
        let result = parallel_multi_source_bfs(&graph, &sources, 9);
        
        assert!(result.is_ok());
        let (source, path) = result.unwrap().unwrap();
        assert!(sources.contains(&source));
        assert!(!path.is_empty());
    }

    #[test]
    fn test_parallel_shortest_paths() {
        let graph = create_test_graph();
        let destinations = vec![5, 6, 7];
        let results = parallel_shortest_paths(&graph, 0, &destinations).unwrap();
        
        assert_eq!(results.len(), 3);
        for dest in destinations {
            assert!(results.contains_key(&dest));
        }
    }

    #[test]
    fn test_parallel_node_degrees() {
        let graph = create_test_graph();
        let degrees = parallel_node_degrees(&graph).unwrap();
        
        assert_eq!(degrees.len(), 10);
        assert_eq!(degrees[&0], 1); // First node has 1 neighbor
        assert_eq!(degrees[&9], 1); // Last node has 1 neighbor
        assert_eq!(degrees[&5], 2); // Middle nodes have 2 neighbors
    }

    #[test]
    fn test_parallel_pagerank() {
        let graph = create_test_graph();
        let ranks = parallel_pagerank(&graph, 0.85, 100, 1e-6).unwrap();
        
        assert_eq!(ranks.len(), 10);
        
        let sum: f64 = ranks.values().sum();
        assert!((sum - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_parallel_k_hop_neighbors() {
        let graph = create_test_graph();
        let sources = vec![0, 5];
        let results = parallel_k_hop_neighbors(&graph, &sources, 2).unwrap();
        
        assert_eq!(results.len(), 2);
        assert!(results[&0].contains(&1));
        assert!(results[&0].contains(&2));
    }
}
