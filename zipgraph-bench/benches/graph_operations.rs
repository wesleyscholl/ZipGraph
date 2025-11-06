use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use zipgraph_core::Graph;

fn create_random_graph(node_count: usize, edge_density: f64) -> Graph {
    let mut graph = Graph::with_capacity(node_count, (node_count as f64 * edge_density) as usize);
    let mut rng = rand::thread_rng();

    for i in 0..node_count {
        graph.add_node_simple(format!("Node{}", i));
    }

    for i in 0..node_count {
        for j in i + 1..node_count {
            if rng.gen::<f64>() < edge_density {
                let weight = rng.gen_range(1.0..10.0);
                let _ = graph.add_edge(i, j, weight);
            }
        }
    }

    graph
}

fn bench_graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_creation");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut graph = Graph::with_capacity(size, size * 2);
                for i in 0..size {
                    graph.add_node_simple(format!("Node{}", i));
                }
                black_box(graph)
            });
        });
    }
    group.finish();
}

fn bench_edge_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_addition");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut graph = Graph::with_capacity(size, size * 2);
                    for i in 0..size {
                        graph.add_node_simple(format!("Node{}", i));
                    }
                    graph
                },
                |mut graph| {
                    for i in 0..size - 1 {
                        let _ = graph.add_edge(i, i + 1, 1.0);
                    }
                    black_box(graph)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_neighbor_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("neighbor_lookup");
    
    for size in [100, 1000, 10000].iter() {
        let graph = create_random_graph(*size, 0.1);
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let neighbors = graph.neighbors(0).unwrap();
                black_box(neighbors)
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_graph_creation,
    bench_edge_addition,
    bench_neighbor_lookup
);
criterion_main!(benches);
