# Changelog

All notable changes to ZipGraph will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-06

### 🎉 Major Release - Ultra Performance Edition

#### Added - Ultra-Optimized Algorithms
- **Lock-Free BFS** (`ultra_bfs`): Atomic operations for thread-safe traversal
- **Batch Processing** (`batch_bfs`): 23-64x speedup for multiple queries
- **Ultra PageRank** (`ultra_pagerank`): Cache-friendly flat arrays (3-5x faster)
- **Batch Shortest Paths** (`batch_shortest_paths`): Shared computation for efficiency
- **Zero-Copy Iterator** (`zero_copy_bfs`): Memory-efficient graph traversal

#### Added - Enterprise Metrics System
- Automatic operation timing with RAII pattern (`OperationTimer`)
- Percentile tracking (p50, p95, p99 latencies)
- Cache hit/miss analytics with thread-safe counters
- JSON-serializable performance metrics
- Global metrics singleton with `once_cell`

#### Added - Persistent Storage
- **Binary format**: Fast serialization with bincode
- **JSON format**: Human-readable with serde_json
- **GraphML format**: XML-based graph interchange (fully working)
- Automatic format detection and error handling

#### Performance Achievements
- **Batch BFS**: 23-64x faster than sequential execution
- **Zero-Copy Iteration**: <0.14ms for 5000 nodes
- **Lock-Free Operations**: No mutex contention
- **Cache Optimization**: 3-5x improvement from flat arrays

#### Testing
- **32 core tests** passing (100% pass rate)
- **53 total tests** across workspace (100% pass rate)
- GraphML parser fixed for proper XML node/edge ID extraction

#### Documentation
- Complete v1.0 feature documentation (V1.0_COMPLETE.md)
- Performance benchmark reports with proven speedups
- Demo instructions for all benchmarks
- Error resolution guide

#### Examples
- `ultra_benchmark.rs` - Demonstrates all optimizations
- `performance_comparison.rs` - NetworkX baseline comparison

### Fixed
- GraphML XML parser for correct node/edge ID extraction with 'n' prefix
- All compilation warnings removed (unused variables)

## [0.2.0] - 2025-11-06

### Added
- **PageRank Algorithm**: Iterative PageRank implementation with configurable damping factor and convergence detection in `zipgraph-core/src/centrality.rs`
- **Centrality Measures**:
  - `degree_centrality()`: Normalized degree-based centrality
  - `closeness_centrality()`: Distance-based centrality measure
  - `betweenness_centrality()`: Measures how often nodes lie on shortest paths
- **Parallel Algorithms Module** (`zipgraph-core/src/parallel.rs`):
  - `parallel_multi_source_bfs()`: Multi-source BFS in parallel
  - `parallel_shortest_paths()`: Dijkstra from one source to multiple destinations
  - `parallel_node_degrees()`: Parallel degree computation
  - `parallel_pagerank()`: Parallel PageRank for large graphs
  - `parallel_k_hop_neighbors()`: Find k-hop neighborhoods in parallel
- **Enhanced Node2Vec**:
  - Random walk generation with configurable parameters (p, q)
  - Walk generation across entire graph
  - Simplified co-occurrence-based embedding training
  - Support for biased random walks
- **Test Coverage Framework**:
  - Configured Tarpaulin with 80% coverage threshold
  - HTML and Lcov output formats
  - Workspace-wide coverage tracking
  - Current coverage: **79.05%** (532/673 lines)

### Improved
- Node2Vec trainer now generates proper random walks instead of placeholder implementation
- Added comprehensive test coverage for all new features:
  - 4 tests for centrality measures
  - 5 tests for parallel algorithms
  - 6 tests for Node2Vec functionality
- Fixed all compiler warnings (dead code, unused variables)
- Updated documentation examples to match current API

### Fixed
- Corrected doctest in `zipgraph-core/src/lib.rs` to use actual API methods
- Fixed lifetime issues in parallel algorithms
- Added proper error handling for empty graphs in Node2Vec training

### Test Results
- **Total Tests**: 42 tests across 4 crates
  - zipgraph-core: 20 tests ✓
  - zipgraph-ml: 14 tests ✓
  - zipgraph-optimizer: 7 tests ✓
  - doctests: 4 tests ✓
- **Coverage**: 79.05% (532/673 lines covered)
- **Build**: Clean with no warnings or errors

## [0.1.0] - 2025-11-06

### Added
- Initial project structure with 4 Cargo workspace crates
- **zipgraph-core**: Core graph library with BFS, DFS, Dijkstra algorithms
- **zipgraph-ml**: ML components with embeddings, algorithm selector, anomaly detection
- **zipgraph-optimizer**: Query optimizer with caching and query planning
- **zipgraph-bench**: Benchmark suite with Criterion
- Graph data structures using HashMap-based adjacency lists
- Comprehensive documentation in `docs/` directory
- 4 example applications demonstrating various use cases
- Serde support for graph serialization
- 23 initial unit tests across all crates

### Dependencies
- petgraph 0.6 for graph algorithms reference
- ndarray 0.15 with serde feature for ML operations
- rayon 1.10 for parallelism
- criterion 0.5 for benchmarking
- dashmap 6.0 for concurrent data structures
- parking_lot 0.12 for efficient locking

[0.2.0]: https://github.com/wesleyscholl/ZipGraph/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/wesleyscholl/ZipGraph/releases/tag/v0.1.0
