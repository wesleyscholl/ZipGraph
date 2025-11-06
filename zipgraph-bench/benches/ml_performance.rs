use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use zipgraph_core::Graph;
use zipgraph_ml::{AlgorithmSelector, AnomalyDetector, NodeEmbeddings};
use zipgraph_optimizer::QueryOptimizer;

fn create_test_graph(size: usize) -> Graph {
    let mut graph = Graph::with_capacity(size, size * 2);
    
    for i in 0..size {
        graph.add_node_simple(format!("Node{}", i));
    }

    for i in 0..size - 1 {
        let _ = graph.add_edge(i, i + 1, 1.0);
    }

    graph
}

fn bench_algorithm_selection(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithm_selection");
    
    for size in [100, 500, 1000].iter() {
        let graph = create_test_graph(*size);
        let selector = AlgorithmSelector::new();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let algorithm = selector.select(&graph);
                black_box(algorithm)
            });
        });
    }
    group.finish();
}

fn bench_embeddings(c: &mut Criterion) {
    let mut group = c.benchmark_group("node_embeddings");
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let embeddings = NodeEmbeddings::new(size, 64);
                black_box(embeddings)
            });
        });
    }
    group.finish();
}

fn bench_anomaly_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("anomaly_detection");
    
    for size in [100, 500, 1000].iter() {
        let graph = create_test_graph(*size);
        let detector = AnomalyDetector::new();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let anomalies = detector.detect(&graph);
                black_box(anomalies)
            });
        });
    }
    group.finish();
}

fn bench_query_optimizer(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_optimizer");
    
    for size in [100, 500, 1000].iter() {
        let graph = create_test_graph(*size);
        let mut optimizer = QueryOptimizer::new();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let path = optimizer.shortest_path(&graph, 0, size - 1);
                black_box(path)
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_algorithm_selection,
    bench_embeddings,
    bench_anomaly_detection,
    bench_query_optimizer
);
criterion_main!(benches);
