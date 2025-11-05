# Contributing to ZipGraph

Thank you for your interest in contributing to ZipGraph! This document provides guidelines for contributing.

## 🚀 Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/zipgraph.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test --all`
6. Run benchmarks: `cargo bench`
7. Commit your changes: `git commit -am 'Add new feature'`
8. Push to your fork: `git push origin feature/your-feature-name`
9. Create a Pull Request

## 🧪 Testing

All contributions should include tests:

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p zipgraph-core

# Run with all features
cargo test --all-features
```

## 📊 Benchmarks

Performance is critical. Run benchmarks to ensure no regressions:

```bash
# Run all benchmarks
cargo bench

# Compare with baseline
cargo bench --bench comparison
```

## 📝 Code Style

- Follow Rust best practices and idioms
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Keep functions small and focused
- Add documentation comments for public APIs

## 🎯 Commit Messages

Follow conventional commits:

- `feat: Add new algorithm selector`
- `fix: Resolve memory leak in graph loading`
- `docs: Update installation instructions`
- `perf: Optimize BFS traversal`
- `test: Add integration tests for ML components`

## 🏗️ Architecture Guidelines

### Core (`zipgraph-core`)
- No external ML dependencies
- Focus on performance and correctness
- Comprehensive unit tests
- Zero-cost abstractions

### ML (`zipgraph-ml`)
- Use established ML libraries (candle, tract)
- Document model architectures
- Include training scripts
- Provide pre-trained models

### Optimizer (`zipgraph-optimizer`)
- Profile-guided optimization
- Adaptive algorithms
- Cache-friendly data structures
- Minimal allocation in hot paths

## 🐛 Bug Reports

Include:
- Rust version (`rustc --version`)
- Operating system
- Minimal reproducible example
- Expected vs actual behavior
- Stack trace if applicable

## 💡 Feature Requests

Before proposing a new feature:
1. Check existing issues
2. Discuss in GitHub Discussions
3. Provide use case and rationale
4. Consider performance implications

## 📄 Documentation

- Document all public APIs
- Include code examples
- Update README for major features
- Add to relevant guides in `docs/`

## 🤝 Code of Conduct

Be respectful and inclusive. We're building this together!

## ❓ Questions?

Open a GitHub Discussion or reach out to maintainers.

---

Thank you for making ZipGraph better! ⚡
