//! Graph algorithms

use crate::error::{GraphError, Result};
use crate::graph::Graph;
use crate::types::{NodeId, Weight};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

/// Priority queue item for Dijkstra's algorithm
#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: Weight,
    node: NodeId,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Breadth-First Search
pub fn bfs(graph: &Graph, start: NodeId, goal: NodeId) -> Result<Vec<NodeId>> {
    if !graph.node_ids().contains(&start) {
        return Err(GraphError::NodeNotFound(start));
    }
    if !graph.node_ids().contains(&goal) {
        return Err(GraphError::NodeNotFound(goal));
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            return Ok(reconstruct_path(&parent, start, goal));
        }

        for neighbor in graph.neighbors(current)? {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    Err(GraphError::AlgorithmError(format!(
        "No path from {} to {}",
        start, goal
    )))
}

/// Depth-First Search
pub fn dfs(graph: &Graph, start: NodeId, goal: NodeId) -> Result<Vec<NodeId>> {
    if !graph.node_ids().contains(&start) {
        return Err(GraphError::NodeNotFound(start));
    }
    if !graph.node_ids().contains(&goal) {
        return Err(GraphError::NodeNotFound(goal));
    }

    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    visited.insert(start);

    while let Some(current) = stack.pop() {
        if current == goal {
            return Ok(reconstruct_path(&parent, start, goal));
        }

        for neighbor in graph.neighbors(current)? {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent.insert(neighbor, current);
                stack.push(neighbor);
            }
        }
    }

    Err(GraphError::AlgorithmError(format!(
        "No path from {} to {}",
        start, goal
    )))
}

/// Dijkstra's shortest path algorithm
pub fn dijkstra(graph: &Graph, start: NodeId, goal: NodeId) -> Result<(Vec<NodeId>, Weight)> {
    if !graph.node_ids().contains(&start) {
        return Err(GraphError::NodeNotFound(start));
    }
    if !graph.node_ids().contains(&goal) {
        return Err(GraphError::NodeNotFound(goal));
    }

    let mut dist: HashMap<NodeId, Weight> = HashMap::new();
    let mut parent: HashMap<NodeId, NodeId> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0.0);
    heap.push(State {
        cost: 0.0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node == goal {
            let path = reconstruct_path(&parent, start, goal);
            return Ok((path, cost));
        }

        if cost > *dist.get(&node).unwrap_or(&Weight::INFINITY) {
            continue;
        }

        for (neighbor, weight) in graph.neighbors_with_weights(node)? {
            let next_cost = cost + weight;
            let neighbor_dist = *dist.get(&neighbor).unwrap_or(&Weight::INFINITY);

            if next_cost < neighbor_dist {
                dist.insert(neighbor, next_cost);
                parent.insert(neighbor, node);
                heap.push(State {
                    cost: next_cost,
                    node: neighbor,
                });
            }
        }
    }

    Err(GraphError::AlgorithmError(format!(
        "No path from {} to {}",
        start, goal
    )))
}

/// Reconstruct path from parent map
fn reconstruct_path(
    parent: &HashMap<NodeId, NodeId>,
    start: NodeId,
    goal: NodeId,
) -> Vec<NodeId> {
    let mut path = vec![goal];
    let mut current = goal;

    while current != start {
        if let Some(&prev) = parent.get(&current) {
            path.push(prev);
            current = prev;
        } else {
            break;
        }
    }

    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();
        let n0 = graph.add_node_simple("A");
        let n1 = graph.add_node_simple("B");
        let n2 = graph.add_node_simple("C");
        let n3 = graph.add_node_simple("D");

        graph.add_edge(n0, n1, 1.0).unwrap();
        graph.add_edge(n1, n2, 2.0).unwrap();
        graph.add_edge(n0, n3, 4.0).unwrap();
        graph.add_edge(n3, n2, 1.0).unwrap();

        graph
    }

    #[test]
    fn test_bfs() {
        let graph = create_test_graph();
        let path = bfs(&graph, 0, 2).unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], 0);
        assert_eq!(path[path.len() - 1], 2);
    }

    #[test]
    fn test_dijkstra() {
        let graph = create_test_graph();
        let (path, cost) = dijkstra(&graph, 0, 2).unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], 0);
        assert_eq!(path[path.len() - 1], 2);
        assert!(cost > 0.0);
    }
}
