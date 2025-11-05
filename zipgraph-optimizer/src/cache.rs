//! Intelligent caching for query results

use crate::query::{Query, QueryResult};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Cached query result with metadata
#[derive(Debug, Clone)]
struct CachedResult {
    result: QueryResult,
    execution_time: Duration,
    hit_count: usize,
    last_access: Instant,
}

/// Query cache with adaptive eviction
pub struct QueryCache {
    cache: Arc<DashMap<u64, CachedResult>>,
    max_size: usize,
}

impl QueryCache {
    /// Create a new cache with specified max size
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            max_size,
        }
    }

    /// Get a cached result
    pub fn get(&self, query: &Query) -> Option<QueryResult> {
        let fingerprint = query.fingerprint();
        
        self.cache.get_mut(&fingerprint).map(|mut entry| {
            entry.hit_count += 1;
            entry.last_access = Instant::now();
            entry.result.clone()
        })
    }

    /// Insert a result into the cache
    pub fn insert(&self, query: &Query, result: QueryResult, execution_time: Duration) {
        let fingerprint = query.fingerprint();
        
        // Check if we need to evict
        if self.cache.len() >= self.max_size {
            self.evict_lru();
        }

        let cached = CachedResult {
            result,
            execution_time,
            hit_count: 0,
            last_access: Instant::now(),
        };

        self.cache.insert(fingerprint, cached);
    }

    /// Evict least recently used entry
    fn evict_lru(&self) {
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();

        for entry in self.cache.iter() {
            if entry.value().last_access < oldest_time {
                oldest_time = entry.value().last_access;
                oldest_key = Some(*entry.key());
            }
        }

        if let Some(key) = oldest_key {
            self.cache.remove(&key);
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let mut total_hits = 0;
        let mut total_time = Duration::default();

        for entry in self.cache.iter() {
            total_hits += entry.value().hit_count;
            total_time += entry.value().execution_time;
        }

        CacheStats {
            size: self.cache.len(),
            total_hits,
            avg_execution_time: if self.cache.len() > 0 {
                total_time / self.cache.len() as u32
            } else {
                Duration::default()
            },
        }
    }

    /// Clear the cache
    pub fn clear(&self) {
        self.cache.clear();
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub total_hits: usize,
    pub avg_execution_time: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_creation() {
        let cache = QueryCache::new(100);
        let stats = cache.stats();
        assert_eq!(stats.size, 0);
    }

    #[test]
    fn test_cache_insert_and_get() {
        let cache = QueryCache::new(100);
        let query = Query::Neighbors { node: 1 };
        let result = QueryResult::Neighbors(vec![2, 3, 4]);

        cache.insert(&query, result.clone(), Duration::from_millis(10));
        
        let cached = cache.get(&query);
        assert!(cached.is_some());
    }

    #[test]
    fn test_cache_eviction() {
        let cache = QueryCache::new(2);
        
        let q1 = Query::Neighbors { node: 1 };
        let q2 = Query::Neighbors { node: 2 };
        let q3 = Query::Neighbors { node: 3 };

        cache.insert(&q1, QueryResult::Neighbors(vec![]), Duration::from_millis(1));
        cache.insert(&q2, QueryResult::Neighbors(vec![]), Duration::from_millis(1));
        cache.insert(&q3, QueryResult::Neighbors(vec![]), Duration::from_millis(1));

        let stats = cache.stats();
        assert_eq!(stats.size, 2); // Should have evicted one
    }
}
