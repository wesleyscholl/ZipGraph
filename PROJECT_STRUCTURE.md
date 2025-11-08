# ZipGraph Project Structure

## 📁 Directory Layout

```
ZipGraph/
├── README.md                    # Main project documentation
├── CHANGELOG.md                 # Version history
├── LICENSE                      # MIT License
├── Cargo.toml                   # Workspace configuration
├── Cargo.lock                   # Dependency lock file
│
├── docs/                        # Documentation
│   ├── QUICK_START.md          # Getting started guide
│   ├── CONTRIBUTING.md         # Contribution guidelines
│   ├── architecture.md         # Architecture overview
│   ├── api.md                  # API documentation
│   ├── ml-components.md        # ML features
│   ├── performance.md          # Performance tuning
│   │
│   ├── demo/                   # Demo documentation
│   │   ├── DEMO_GUIDE.md       # Complete demo recording guide
│   │   ├── DEMO_READY.md       # Quick demo checklist
│   │   ├── DEMO_REFERENCE.md   # Quick reference card
│   │   └── DEMO_INSTRUCTIONS.md # Step-by-step instructions
│   │
│   └── release-notes/          # Release documentation
│       ├── RELEASE_SUMMARY.md  # v1.0.0 summary
│       ├── RELEASE_PLAN.md     # Release strategy
│       ├── V1.0_COMPLETE.md    # v1.0 completion notes
│       ├── V0.3_COMPLETE.md    # v0.3 completion notes
│       └── ...                 # Other release notes
│
├── scripts/                     # Utility scripts
│   ├── demo.sh                 # Performance demo script
│   ├── build-release.sh        # Release build script
│   ├── quick_start.sh          # Quick start helper
│   └── fix-github-lang.sh      # GitHub language detection fix
│
├── examples/                    # Example applications
│   ├── ultra_benchmark.rs      # Performance benchmarks
│   ├── performance_comparison.rs
│   ├── basic_usage.rs          # Simple usage example
│   ├── fraud_detection.rs      # Fraud detection demo
│   ├── social_network.rs       # Social network analysis
│   └── recommendation_engine.rs
│
├── zipgraph-core/              # Core graph library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── graph.rs            # Graph data structures
│   │   ├── algorithms.rs       # Classic algorithms
│   │   ├── ultra.rs            # Ultra-optimized algorithms
│   │   ├── metrics.rs          # Enterprise metrics
│   │   ├── storage.rs          # Persistent storage
│   │   └── ...
│   ├── tests/
│   └── Cargo.toml
│
├── zipgraph-ml/                # Machine learning components
│   ├── src/
│   │   ├── lib.rs
│   │   ├── embeddings.rs       # Node embeddings
│   │   ├── algorithm_selector.rs
│   │   └── ...
│   └── Cargo.toml
│
├── zipgraph-optimizer/         # Query optimization
│   ├── src/
│   │   ├── lib.rs
│   │   ├── query.rs
│   │   ├── learned_index.rs
│   │   └── ...
│   └── Cargo.toml
│
├── zipgraph-bench/             # Benchmarks
│   ├── benches/
│   └── Cargo.toml
│
├── benchmarks/                  # Benchmark data and results
├── release/                     # Release artifacts
└── target/                      # Build output (gitignored)
```

## 📄 Key Files

### Root Level
- `README.md` - Project overview, features, quick start
- `CHANGELOG.md` - Complete version history
- `LICENSE` - MIT License text
- `Cargo.toml` - Workspace configuration
- `.gitignore` - Git ignore patterns
- `.gitattributes` - GitHub language detection

### Documentation (`docs/`)
- `QUICK_START.md` - 5-minute getting started guide
- `CONTRIBUTING.md` - How to contribute
- `architecture.md` - System design and architecture
- `api.md` - API reference documentation
- `ml-components.md` - ML features and usage
- `performance.md` - Performance tuning guide

### Demo Documentation (`docs/demo/`)
- `DEMO_GUIDE.md` - Complete guide for recording performance demos
- `DEMO_READY.md` - Quick checklist before recording
- `DEMO_REFERENCE.md` - Quick reference card
- `DEMO_INSTRUCTIONS.md` - Step-by-step demo instructions

### Release Notes (`docs/release-notes/`)
- `RELEASE_SUMMARY.md` - Executive summary of v1.0.0
- `RELEASE_PLAN.md` - Release strategy and timeline
- Version-specific completion notes and summaries

### Scripts (`scripts/`)
- `demo.sh` - Automated 30-second performance demo
- `build-release.sh` - Build release binaries with optimizations
- `quick_start.sh` - Quick start helper script
- `fix-github-lang.sh` - Fix GitHub language detection

### Examples (`examples/`)
Runnable examples demonstrating various features:
- Performance benchmarks
- Basic usage patterns
- Real-world applications (fraud, social networks, etc.)

## 🏗️ Crate Structure

### `zipgraph-core`
Core graph library with:
- Graph data structures
- Classic algorithms (BFS, DFS, Dijkstra, PageRank)
- Ultra-optimized algorithms (43-70x speedup)
- Enterprise metrics and monitoring
- Persistent storage (Binary, JSON, GraphML)

### `zipgraph-ml`
Machine learning components:
- Node embeddings (Node2Vec, GraphSAGE)
- Algorithm selection
- Anomaly detection
- Pattern learning

### `zipgraph-optimizer`
Query optimization:
- Intelligent query planning
- Learned index structures
- Adaptive caching
- Cost estimation

### `zipgraph-bench`
Performance benchmarking:
- Criterion benchmarks
- Scalability tests
- Memory profiling

## 🚀 Quick Navigation

| Task | Location |
|------|----------|
| Get started | `docs/QUICK_START.md` |
| Run demo | `./scripts/demo.sh` |
| See examples | `examples/` |
| Read docs | `docs/` |
| View release notes | `docs/release-notes/` |
| Check changelog | `CHANGELOG.md` |
| Contribute | `docs/CONTRIBUTING.md` |

## 📦 Build Artifacts

| Directory | Purpose | Gitignored |
|-----------|---------|------------|
| `target/` | Cargo build output | ✅ Yes |
| `release/` | Release binaries | ❌ No |
| `benchmarks/` | Benchmark results | ❌ No |
| `coverage/` | Test coverage reports | ✅ Yes |

## 🧹 Clean Repository

The repository is organized to keep the root clean:
- ✅ Only essential files in root (README, LICENSE, Cargo files)
- ✅ Documentation organized in `docs/`
- ✅ Scripts in `scripts/`
- ✅ Examples in `examples/`
- ✅ Source code in crate directories
- ✅ Release notes archived in `docs/release-notes/`

This structure makes it easy to:
- Find documentation quickly
- Run scripts from consistent location
- Navigate source code
- Maintain clean git history
- Onboard new contributors
