//! Query optimizer with ML-powered planning

use crate::cache::QueryCache;
use crate::error::{OptimizerError, Result};
use crate::query::{Query, QueryResult};
use std::time::Instant;
use zipgraph_core::{algorithms, Graph, NodeId};
use zipgraph_ml::AlgorithmSelector;

/// Main query optimizer
pub struct QueryOptimizer {
    cache: QueryCache,
    algorithm_selector: AlgorithmSelector,
    stats: OptimizerStats,
}

#[derive(Debug, Default)]
struct OptimizerStats {
    queries_executed: usize,
    cache_hits: usize,
    cache_misses: usize,
}

impl QueryOptimizer {
    /// Create a new query optimizer
    pub fn new() -> Self {
        Self::with_cache_size(1000)
    }

    /// Create optimizer with specified cache size
    pub fn with_cache_size(cache_size: usize) -> Self {
        Self {
            cache: QueryCache::new(cache_size),
            algorithm_selector: AlgorithmSelector::new(),
            stats: OptimizerStats::default(),
        }
    }

    /// Execute a query with optimization
    pub fn execute(&mut self, graph: &Graph, query: &Query) -> Result<QueryResult> {
        // Check cache first
        if let Some(cached_result) = self.cache.get(query) {
            self.stats.cache_hits += 1;
            return Ok(cached_result);
        }

        self.stats.cache_misses += 1;
        self.stats.queries_executed += 1;

        // Execute query
        let start = Instant::now();
        let result = self.execute_query(graph, query)?;
        let execution_time = start.elapsed();

        // Cache the result
        self.cache.insert(query, result.clone(), execution_time);

        Ok(result)
    }

    /// Execute shortest path query
    pub fn shortest_path(
        &mut self,
        graph: &Graph,
        start: NodeId,
        goal: NodeId,
    ) -> Result<Vec<NodeId>> {
        let query = Query::ShortestPath { start, goal };
        
        match self.execute(graph, &query)? {
            QueryResult::Path(path) => Ok(path),
            _ => Err(OptimizerError::ExecutionError(
                "Unexpected result type".to_string(),
            )),
        }
    }

    /// Execute neighbors query
    pub fn neighbors(&mut self, graph: &Graph, node: NodeId) -> Result<Vec<NodeId>> {
        let query = Query::Neighbors { node };
        
        match self.execute(graph, &query)? {
            QueryResult::Neighbors(neighbors) => Ok(neighbors),
            _ => Err(OptimizerError::ExecutionError(
                "Unexpected result type".to_string(),
            )),
        }
    }

    /// Internal query execution
    fn execute_query(&self, graph: &Graph, query: &Query) -> Result<QueryResult> {
        match query {
            Query::ShortestPath { start, goal } => {
                // Use ML to select best algorithm
                let algorithm = self.algorithm_selector.select_shortest_path(graph, *start, *goal);
                
                let path = match algorithm {
                    zipgraph_core::Algorithm::BFS => algorithms::bfs(graph, *start, *goal)?,
                    zipgraph_core::Algorithm::DFS => algorithms::dfs(graph, *start, *goal)?,
                    zipgraph_core::Algorithm::Dijkstra => {
                        let (path, _cost) = algorithms::dijkstra(graph, *start, *goal)?;
                        path
                    }
                    _ => algorithms::dijkstra(graph, *start, *goal)?.0,
                };
                
                Ok(QueryResult::Path(path))
            }
            Query::Neighbors { node } => {
                let neighbors = graph.neighbors(*node)?;
                Ok(QueryResult::Neighbors(neighbors))
            }
            Query::PageRank { .. } => {
                // TODO: Implement PageRank
                Ok(QueryResult::Scores(vec![]))
            }
            Query::ConnectedComponents => {
                // TODO: Implement connected components
                Ok(QueryResult::Components(vec![]))
            }
            Query::ShortestPaths { .. } => {
                // TODO: Implement all-pairs shortest paths
                Ok(QueryResult::Path(vec![]))
            }
        }
    }

    /// Get optimizer statistics
    pub fn stats(&self) -> String {
        format!(
            "Queries: {}, Cache hits: {}, Cache misses: {}, Hit rate: {:.2}%",
            self.stats.queries_executed,
            self.stats.cache_hits,
            self.stats.cache_misses,
            if self.stats.queries_executed > 0 {
                (self.stats.cache_hits as f64 / (self.stats.cache_hits + self.stats.cache_misses) as f64) * 100.0
            } else {
                0.0
            }
        )
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for QueryOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();
        let n0 = graph.add_node_simple("A");
        let n1 = graph.add_node_simple("B");
        let n2 = graph.add_node_simple("C");

        graph.add_edge(n0, n1, 1.0).unwrap();
        graph.add_edge(n1, n2, 2.0).unwrap();

        graph
    }

    #[test]
    fn test_optimizer_creation() {
        let optimizer = QueryOptimizer::new();
        assert!(optimizer.stats().contains("Queries: 0"));
    }

    #[test]
    fn test_shortest_path_query() {
        let mut optimizer = QueryOptimizer::new();
        let graph = create_test_graph();

        let path = optimizer.shortest_path(&graph, 0, 2).unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], 0);
        assert_eq!(path[path.len() - 1], 2);
    }

    #[test]
    fn test_neighbors_query() {
        let mut optimizer = QueryOptimizer::new();
        let graph = create_test_graph();

        let neighbors = optimizer.neighbors(&graph, 1).unwrap();
        assert!(!neighbors.is_empty());
    }

    #[test]
    fn test_caching() {
        let mut optimizer = QueryOptimizer::new();
        let graph = create_test_graph();

        // First query - cache miss
        optimizer.shortest_path(&graph, 0, 2).unwrap();
        
        // Second query - cache hit
        optimizer.shortest_path(&graph, 0, 2).unwrap();

        assert!(optimizer.stats().contains("Cache hits: 1"));
    }
}
