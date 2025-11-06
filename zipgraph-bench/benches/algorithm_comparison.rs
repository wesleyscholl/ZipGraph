use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use zipgraph_core::{algorithms, Graph};

fn create_test_graph(size: usize) -> Graph {
    let mut graph = Graph::with_capacity(size, size * 2);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        graph.add_node_simple(format!("Node{}", i));
    }

    for i in 0..size - 1 {
        let weight = rng.gen_range(1.0..10.0);
        let _ = graph.add_edge(i, i + 1, weight);
    }

    // Add some random edges for complexity
    for _ in 0..size / 2 {
        let from = rng.gen_range(0..size);
        let to = rng.gen_range(0..size);
        if from != to {
            let weight = rng.gen_range(1.0..10.0);
            let _ = graph.add_edge(from, to, weight);
        }
    }

    graph
}

fn bench_bfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("bfs");
    
    for size in [100, 500, 1000].iter() {
        let graph = create_test_graph(*size);
        let goal = size - 1;
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let path = algorithms::bfs(&graph, 0, goal);
                black_box(path)
            });
        });
    }
    group.finish();
}

fn bench_dfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs");
    
    for size in [100, 500, 1000].iter() {
        let graph = create_test_graph(*size);
        let goal = size - 1;
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let path = algorithms::dfs(&graph, 0, goal);
                black_box(path)
            });
        });
    }
    group.finish();
}

fn bench_dijkstra(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra");
    
    for size in [100, 500, 1000].iter() {
        let graph = create_test_graph(*size);
        let goal = size - 1;
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = algorithms::dijkstra(&graph, 0, goal);
                black_box(result)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bfs, bench_dfs, bench_dijkstra);
criterion_main!(benches);
