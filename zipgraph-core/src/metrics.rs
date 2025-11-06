//! Enterprise monitoring and metrics for ZipGraph
//!
//! Provides real-time performance monitoring, telemetry, and diagnostics

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Global metrics collector
static METRICS: once_cell::sync::Lazy<Metrics> = once_cell::sync::Lazy::new(Metrics::new);

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation: String,
    pub count: u64,
    pub total_duration_ms: f64,
    pub avg_duration_ms: f64,
    pub min_duration_ms: f64,
    pub max_duration_ms: f64,
    pub p50_duration_ms: f64,
    pub p95_duration_ms: f64,
    pub p99_duration_ms: f64,
}

/// Operation timer for automatic metric collection
pub struct OperationTimer {
    operation: String,
    start: Instant,
}

impl OperationTimer {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            start: Instant::now(),
        }
    }
}

impl Drop for OperationTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        METRICS.record_operation(&self.operation, duration);
    }
}

/// Metrics storage and aggregation
pub struct Metrics {
    operations: Arc<RwLock<HashMap<String, OperationMetrics>>>,
    graph_operations: AtomicU64,
    total_nodes_processed: AtomicU64,
    total_edges_processed: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

#[derive(Debug)]
struct OperationMetrics {
    count: AtomicUsize,
    total_duration_ns: AtomicU64,
    min_duration_ns: AtomicU64,
    max_duration_ns: AtomicU64,
    durations: RwLock<Vec<u64>>, // For percentile calculation
}

impl OperationMetrics {
    fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
            total_duration_ns: AtomicU64::new(0),
            min_duration_ns: AtomicU64::new(u64::MAX),
            max_duration_ns: AtomicU64::new(0),
            durations: RwLock::new(Vec::new()),
        }
    }

    fn record(&self, duration: Duration) {
        let nanos = duration.as_nanos() as u64;
        
        self.count.fetch_add(1, Ordering::Relaxed);
        self.total_duration_ns.fetch_add(nanos, Ordering::Relaxed);
        
        // Update min
        let mut current_min = self.min_duration_ns.load(Ordering::Relaxed);
        while nanos < current_min {
            match self.min_duration_ns.compare_exchange_weak(
                current_min,
                nanos,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_min = x,
            }
        }
        
        // Update max
        let mut current_max = self.max_duration_ns.load(Ordering::Relaxed);
        while nanos > current_max {
            match self.max_duration_ns.compare_exchange_weak(
                current_max,
                nanos,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_max = x,
            }
        }
        
        // Store for percentile calculation (with sampling to limit memory)
        let count = self.count.load(Ordering::Relaxed);
        if count < 10000 || count % 100 == 0 {
            self.durations.write().push(nanos);
        }
    }

    fn to_performance_metrics(&self, operation: String) -> PerformanceMetrics {
        let count = self.count.load(Ordering::Relaxed) as u64;
        let total_ns = self.total_duration_ns.load(Ordering::Relaxed);
        let min_ns = self.min_duration_ns.load(Ordering::Relaxed);
        let max_ns = self.max_duration_ns.load(Ordering::Relaxed);
        
        let avg_ms = if count > 0 {
            (total_ns as f64 / count as f64) / 1_000_000.0
        } else {
            0.0
        };
        
        let mut durations = self.durations.read().clone();
        durations.sort_unstable();
        
        let (p50, p95, p99) = if !durations.is_empty() {
            let len = durations.len();
            let p50_idx = (len as f64 * 0.50) as usize;
            let p95_idx = (len as f64 * 0.95) as usize;
            let p99_idx = (len as f64 * 0.99) as usize;
            
            (
                durations[p50_idx.min(len - 1)] as f64 / 1_000_000.0,
                durations[p95_idx.min(len - 1)] as f64 / 1_000_000.0,
                durations[p99_idx.min(len - 1)] as f64 / 1_000_000.0,
            )
        } else {
            (0.0, 0.0, 0.0)
        };
        
        PerformanceMetrics {
            operation,
            count,
            total_duration_ms: total_ns as f64 / 1_000_000.0,
            avg_duration_ms: avg_ms,
            min_duration_ms: min_ns as f64 / 1_000_000.0,
            max_duration_ms: max_ns as f64 / 1_000_000.0,
            p50_duration_ms: p50,
            p95_duration_ms: p95,
            p99_duration_ms: p99,
        }
    }
}

impl Metrics {
    fn new() -> Self {
        Self {
            operations: Arc::new(RwLock::new(HashMap::new())),
            graph_operations: AtomicU64::new(0),
            total_nodes_processed: AtomicU64::new(0),
            total_edges_processed: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
        }
    }

    fn record_operation(&self, operation: &str, duration: Duration) {
        let mut ops = self.operations.write();
        let metrics = ops
            .entry(operation.to_string())
            .or_insert_with(OperationMetrics::new);
        metrics.record(duration);
    }

    /// Get all performance metrics
    pub fn get_all_metrics(&self) -> Vec<PerformanceMetrics> {
        let ops = self.operations.read();
        ops.iter()
            .map(|(name, metrics)| metrics.to_performance_metrics(name.clone()))
            .collect()
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.operations.write().clear();
        self.graph_operations.store(0, Ordering::Relaxed);
        self.total_nodes_processed.store(0, Ordering::Relaxed);
        self.total_edges_processed.store(0, Ordering::Relaxed);
        self.cache_hits.store(0, Ordering::Relaxed);
        self.cache_misses.store(0, Ordering::Relaxed);
    }

    /// Record graph operation
    pub fn inc_graph_operations(&self) {
        self.graph_operations.fetch_add(1, Ordering::Relaxed);
    }

    /// Record nodes processed
    pub fn add_nodes_processed(&self, count: u64) {
        self.total_nodes_processed.fetch_add(count, Ordering::Relaxed);
    }

    /// Record edges processed
    pub fn add_edges_processed(&self, count: u64) {
        self.total_edges_processed.fetch_add(count, Ordering::Relaxed);
    }

    /// Record cache hit
    pub fn inc_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Record cache miss
    pub fn inc_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    /// Get cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    /// Get summary statistics
    pub fn summary(&self) -> String {
        let ops = self.graph_operations.load(Ordering::Relaxed);
        let nodes = self.total_nodes_processed.load(Ordering::Relaxed);
        let edges = self.total_edges_processed.load(Ordering::Relaxed);
        let hit_rate = self.cache_hit_rate();
        
        format!(
            "Graph Operations: {}\nNodes Processed: {}\nEdges Processed: {}\nCache Hit Rate: {:.2}%",
            ops, nodes, edges, hit_rate * 100.0
        )
    }
}

/// Global metrics access
pub fn metrics() -> &'static Metrics {
    &METRICS
}

/// Create an operation timer (automatic recording on drop)
pub fn timer(operation: impl Into<String>) -> OperationTimer {
    OperationTimer::new(operation)
}

/// Get all performance metrics
pub fn get_metrics() -> Vec<PerformanceMetrics> {
    METRICS.get_all_metrics()
}

/// Reset all metrics
pub fn reset_metrics() {
    METRICS.reset();
}

/// Print metrics summary
pub fn print_summary() {
    println!("{}", METRICS.summary());
    println!("\nOperation Metrics:");
    for metric in get_metrics() {
        println!(
            "  {}: count={}, avg={:.3}ms, p95={:.3}ms, p99={:.3}ms",
            metric.operation,
            metric.count,
            metric.avg_duration_ms,
            metric.p95_duration_ms,
            metric.p99_duration_ms
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_metrics_recording() {
        reset_metrics();
        
        {
            let _timer = timer("test_op");
            thread::sleep(Duration::from_millis(10));
        }
        
        let metrics = get_metrics();
        assert!(!metrics.is_empty());
        
        let test_metric = metrics.iter().find(|m| m.operation == "test_op").unwrap();
        assert_eq!(test_metric.count, 1);
        assert!(test_metric.avg_duration_ms >= 10.0);
    }

    #[test]
    fn test_cache_metrics() {
        reset_metrics();
        
        metrics().inc_cache_hit();
        metrics().inc_cache_hit();
        metrics().inc_cache_miss();
        
        let hit_rate = metrics().cache_hit_rate();
        assert!((hit_rate - 0.6667).abs() < 0.01);
    }
}
