# 🚀 Getting Started with ZipGraph

## Installation

### Prerequisites
- Rust 1.75+ (install from [rustup.rs](https://rustup.rs/))
- Git
- A C compiler (for BLAS/LAPACK dependencies)

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/zipgraph.git
cd zipgraph

# Run the quick start script
./quick_start.sh

# Or build manually
cargo build --release
cargo test --all
```

## Your First Graph

Create a new Rust project and add ZipGraph as a dependency:

```toml
[dependencies]
zipgraph-core = { path = "../zipgraph/zipgraph-core" }
zipgraph-ml = { path = "../zipgraph/zipgraph-ml" }
zipgraph-optimizer = { path = "../zipgraph/zipgraph-optimizer" }
```

### Example 1: Basic Graph Operations

```rust
use zipgraph_core::Graph;

fn main() {
    // Create a new graph
    let mut graph = Graph::new();
    
    // Add nodes
    let node_a = graph.add_node_simple("A");
    let node_b = graph.add_node_simple("B");
    let node_c = graph.add_node_simple("C");
    
    // Add edges with weights
    graph.add_edge(node_a, node_b, 5.0).unwrap();
    graph.add_edge(node_b, node_c, 3.0).unwrap();
    
    // Query the graph
    println!("Nodes: {}", graph.node_count());
    println!("Edges: {}", graph.edge_count());
    
    // Find neighbors
    let neighbors = graph.neighbors(node_b).unwrap();
    println!("Node B has {} neighbors", neighbors.len());
}
```

### Example 2: Shortest Path Finding

```rust
use zipgraph_core::{Graph, algorithms};

fn main() {
    let mut graph = Graph::new();
    
    let a = graph.add_node_simple("City A");
    let b = graph.add_node_simple("City B");
    let c = graph.add_node_simple("City C");
    
    graph.add_edge(a, b, 10.0).unwrap();
    graph.add_edge(b, c, 20.0).unwrap();
    graph.add_edge(a, c, 50.0).unwrap();
    
    // Find shortest path using Dijkstra
    match algorithms::dijkstra(&graph, a, c) {
        Ok((path, cost)) => {
            println!("Path: {:?}", path);
            println!("Total distance: {}", cost);
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

### Example 3: Using the Query Optimizer

```rust
use zipgraph_optimizer::QueryOptimizer;
use zipgraph_core::Graph;

fn main() {
    let mut graph = Graph::new();
    let mut optimizer = QueryOptimizer::new();
    
    // ... build your graph ...
    
    // First query - optimizer learns and caches
    let path1 = optimizer.shortest_path(&graph, 0, 5).unwrap();
    
    // Second identical query - served from cache (faster!)
    let path2 = optimizer.shortest_path(&graph, 0, 5).unwrap();
    
    // Check optimizer statistics
    println!("{}", optimizer.stats());
}
```

### Example 4: ML-Powered Algorithm Selection

```rust
use zipgraph_ml::AlgorithmSelector;
use zipgraph_core::Graph;

fn main() {
    let graph = Graph::new();
    // ... add nodes and edges ...
    
    let selector = AlgorithmSelector::new();
    
    // Let ML choose the best algorithm for your graph
    let algorithm = selector.select(&graph);
    println!("Best algorithm: {:?}", algorithm);
}
```

### Example 5: Anomaly Detection

```rust
use zipgraph_ml::AnomalyDetector;
use zipgraph_core::Graph;

fn main() {
    let mut graph = Graph::new();
    // ... build your graph ...
    
    let mut detector = AnomalyDetector::new();
    detector.train_on_baseline(&graph).unwrap();
    
    // Detect anomalies
    let anomalies = detector.detect(&graph);
    
    for anomaly in anomalies {
        println!("Found anomaly: {:?}", anomaly.reason);
        println!("Score: {:.2}", anomaly.anomaly_score);
    }
}
```

## Running Examples

ZipGraph includes several complete examples:

```bash
# Basic usage
cargo run --example basic_usage

# Real-time recommendation engine
cargo run --example recommendation_engine

# Fraud detection with anomaly detection
cargo run --example fraud_detection

# Social network analysis
cargo run --example social_network
```

## Performance Benchmarks

Run benchmarks to see ZipGraph's performance:

```bash
# All benchmarks
cargo bench

# Specific benchmark
cargo bench --bench graph_operations
cargo bench --bench algorithm_comparison
cargo bench --bench ml_performance
```

## Common Patterns

### Creating a Directed Graph

```rust
let mut graph = Graph::new_directed();
graph.add_edge(0, 1, 1.0).unwrap();
// Edge only goes from 0 -> 1, not bidirectional
```

### Adding Node Properties

```rust
use zipgraph_core::Node;

let node = Node::new(0, "MyNode")
    .with_property("age", 25.0)
    .with_property("score", 0.85);

graph.add_node(node);
```

### Working with Large Graphs

```rust
// Pre-allocate capacity for better performance
let mut graph = Graph::with_capacity(1_000_000, 5_000_000);

// Use parallel operations
use rayon::prelude::*;
let results: Vec<_> = nodes.par_iter()
    .map(|node| process_node(node))
    .collect();
```

### Serializing Graphs

```rust
use serde_json;

// Save to JSON
let json = serde_json::to_string(&graph).unwrap();
std::fs::write("graph.json", json).unwrap();

// Load from JSON
let json = std::fs::read_to_string("graph.json").unwrap();
let graph: Graph = serde_json::from_str(&json).unwrap();
```

## API Reference

### Core Types

- `Graph` - Main graph data structure
- `Node` - Graph node with properties
- `Edge` - Graph edge with weight and type
- `Algorithm` - Algorithm enum for selection

### Main Functions

#### Graph Operations
- `add_node(node)` - Add a node to the graph
- `add_node_simple(label)` - Add a node with just a label
- `add_edge(from, to, weight)` - Add an edge
- `neighbors(node_id)` - Get node neighbors
- `degree(node_id)` - Get node degree

#### Algorithms
- `algorithms::bfs(graph, start, goal)` - Breadth-first search
- `algorithms::dfs(graph, start, goal)` - Depth-first search
- `algorithms::dijkstra(graph, start, goal)` - Shortest path

#### Optimizer
- `optimizer.shortest_path(graph, start, goal)` - Optimized pathfinding
- `optimizer.neighbors(graph, node)` - Cached neighbor lookup
- `optimizer.stats()` - Get cache statistics

#### ML Components
- `selector.select(graph)` - Select best algorithm
- `detector.detect(graph)` - Detect anomalies
- `embeddings.get_embedding(node)` - Get node embedding

## Performance Tips

1. **Pre-allocate capacity** when you know graph size
2. **Use the optimizer** for repeated queries
3. **Batch operations** when possible
4. **Profile first** before optimizing
5. **Enable release mode** for production (`--release`)

## Troubleshooting

### Build Errors

If you encounter BLAS/LAPACK errors:
```bash
# macOS
brew install openblas

# Ubuntu/Debian
sudo apt-get install libopenblas-dev liblapack-dev

# Fedora
sudo dnf install openblas-devel lapack-devel
```

### Performance Issues

- Ensure you're using `--release` mode
- Check if caching is enabled in optimizer
- Profile with `cargo flamegraph` to find bottlenecks

### Memory Issues

- Use `Graph::with_capacity()` to pre-allocate
- Consider streaming large graphs instead of loading all at once
- Monitor with `cargo instruments` on macOS

## Next Steps

- Read the [Architecture Guide](../docs/ARCHITECTURE.md)
- Check out the [Roadmap](../docs/ROADMAP.md)
- Explore [Contributing Guidelines](../CONTRIBUTING.md)
- Join discussions on GitHub

## Resources

- **Documentation**: [docs.rs/zipgraph](https://docs.rs/zipgraph)
- **GitHub**: [github.com/yourusername/zipgraph](https://github.com/yourusername/zipgraph)
- **Examples**: See `examples/` directory
- **Benchmarks**: See `zipgraph-bench/benches/`

## Getting Help

- Open an issue on GitHub
- Check existing issues for solutions
- Read the documentation
- Ask in GitHub Discussions

---

**Happy graphing with ZipGraph!** ⚡🤐
