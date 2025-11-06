# ZipGraph v1.0.0 - Quick Reference Guide

## 🚀 Installation

### Extract the Release
```bash
tar -xzf zipgraph-v1.0.0-arm64-darwin.tar.gz
cd zipgraph-v1.0.0-arm64-darwin
```

### Run the Install Script
```bash
./install.sh
```

### Add to PATH
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## 🎯 Quick Start

### Run Benchmarks
```bash
# Ultra performance benchmark
ultra_benchmark

# NetworkX comparison
performance_comparison
```

### Run Examples
```bash
# Basic usage
basic_usage

# Domain-specific examples
fraud_detection
recommendation_engine
social_network
```

## 📊 Performance Reference

### Batch Processing Speedups
| Queries | Nodes | Sequential | Batch | Speedup |
|---------|-------|-----------|-------|---------|
| 100     | 1000  | 105.8ms   | 3.2ms | **33x** |
| 500     | 5000  | 2677ms    | 37.6ms| **71x** |

### Memory Efficiency
- Zero-copy iteration: <0.14ms for 5000 nodes
- Ultra PageRank: 132ms for 5000 nodes

## 💻 Code Examples

### Basic Graph Creation
```rust
use zipgraph_core::Graph;

let mut graph = Graph::new();
let a = graph.add_node_simple("Alice");
let b = graph.add_node_simple("Bob");
graph.add_edge(a, b, 1.0).unwrap();
```

### Ultra-Fast Batch Processing
```rust
use zipgraph_core::ultra;

// Process multiple queries 23-71x faster!
let queries = vec![(0, 5), (1, 6), (2, 7)];
let results = ultra::batch_bfs(&graph, &queries);
```

### Zero-Copy Iteration
```rust
use zipgraph_core::ultra;

let iter = ultra::zero_copy_bfs(&graph, start);
for node in iter.take(1000) {
    // Process nodes without allocation overhead
}
```

### Enterprise Metrics
```rust
use zipgraph_core::metrics;

{
    let _timer = metrics::timer("operation");
    // Your code - automatically timed
}

metrics::print_summary();
```

### Persistent Storage
```rust
use zipgraph_core::storage::{save_graph, load_graph, StorageFormat};

// Save
save_graph(&graph, "graph.bin", StorageFormat::Binary)?;
save_graph(&graph, "graph.json", StorageFormat::Json)?;
save_graph(&graph, "graph.graphml", StorageFormat::GraphML)?;

// Load
let graph = load_graph("graph.bin", StorageFormat::Binary)?;
```

## 📁 File Locations

After installation:
- Binaries: `~/.local/bin/`
- Libraries: `~/.local/lib/`
- Examples: In release package
- Docs: In release package `docs/` directory

## 🔧 Troubleshooting

### Binary Not Found
```bash
# Check installation
ls ~/.local/bin/ultra_benchmark

# Add to PATH if needed
export PATH="$HOME/.local/bin:$PATH"
```

### Permission Denied
```bash
chmod +x ~/.local/bin/*
```

### Library Issues
```bash
# Check libraries
ls ~/.local/lib/libzipgraph_*

# Set library path if needed
export DYLD_LIBRARY_PATH="$HOME/.local/lib:$DYLD_LIBRARY_PATH"
```

## 📚 Documentation

### In Package
- `docs/README.md` - Overview
- `docs/CHANGELOG.md` - Version history
- `docs/V1.0_COMPLETE.md` - Complete feature docs
- `docs/32_TESTS_PASSING.md` - Test status

### Online
- Repository: https://github.com/wesleyscholl/zipgraph
- API Docs: https://docs.rs/zipgraph

## 🎯 Common Tasks

### Build from Source
```bash
git clone https://github.com/wesleyscholl/zipgraph.git
cd zipgraph
cargo build --release
cargo test --workspace
```

### Run Specific Example
```bash
cargo run --release --example ultra_benchmark
cargo run --release --example basic_usage
```

### Generate Documentation
```bash
cargo doc --workspace --no-deps --open
```

## ⚡ Performance Tips

1. **Use Batch APIs**: 23-71x faster for multiple queries
2. **Use Zero-Copy Iterators**: Minimize allocations
3. **Enable Metrics**: Monitor performance in production
4. **Use Binary Format**: Fastest persistence option
5. **Parallel Algorithms**: Leverage multi-core CPUs

## 🐛 Reporting Issues

Found a bug? Please report it:
1. Check existing issues: https://github.com/wesleyscholl/zipgraph/issues
2. Create new issue with:
   - ZipGraph version (v1.0.0)
   - Platform (macOS arm64)
   - Reproduction steps
   - Expected vs actual behavior

## 📞 Getting Help

- GitHub Issues: Bug reports
- GitHub Discussions: Q&A and community support
- Documentation: Complete API reference

## 🎓 Learning Resources

### Examples in Package
1. `basic_usage` - Start here for fundamentals
2. `fraud_detection` - Learn anomaly detection
3. `recommendation_engine` - Build recommendation systems
4. `social_network` - Analyze social graphs

### Documentation
- README.md - Quick overview
- V1.0_COMPLETE.md - Feature deep dive
- CHANGELOG.md - What's new

---

**Version**: 1.0.0  
**Platform**: macOS arm64 (Apple Silicon)  
**Release Date**: November 6, 2025
