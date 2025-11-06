//! ZipGraph Performance Benchmark
//!
//! Compare ZipGraph performance against NetworkX baseline
//! Run this after running compare_networkx.py

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use zipgraph_core::{centrality::*, parallel::*, Graph};

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkResult {
    nodes: usize,
    edges: usize,
    graph_creation_ms: f64,
    pagerank_ms: f64,
    shortest_paths_ms: f64,
    betweenness_ms: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkSuite {
    library: String,
    version: String,
    benchmarks: Vec<BenchmarkResult>,
}

fn benchmark_graph_creation(n_nodes: usize, n_edges: usize) -> (f64, Graph) {
    let start = Instant::now();
    let mut graph = Graph::new();
    let mut rng = rand::thread_rng();

    // Add nodes
    for i in 0..n_nodes {
        graph.add_node_simple(format!("Node{}", i));
    }

    // Add random edges
    let mut edges_added = 0;
    let mut attempted = HashSet::new();
    
    while edges_added < n_edges {
        let u = rng.gen_range(0..n_nodes);
        let v = rng.gen_range(0..n_nodes);
        
        if u != v && !attempted.contains(&(u.min(v), u.max(v))) {
            attempted.insert((u.min(v), u.max(v)));
            if graph.add_edge(u, v, rng.gen::<f64>()).is_ok() {
                edges_added += 1;
            }
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    (elapsed, graph)
}

fn benchmark_pagerank(graph: &Graph) -> f64 {
    let start = Instant::now();
    let _ = pagerank(graph, 0.85, 100, 1e-6).unwrap();
    start.elapsed().as_secs_f64()
}

fn benchmark_shortest_paths(graph: &Graph, n_samples: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let nodes: Vec<_> = graph.node_ids();
    
    if nodes.len() < 2 {
        return 0.0;
    }

    let start = Instant::now();
    for _ in 0..n_samples.min(nodes.len() / 2) {
        let source = nodes[rng.gen_range(0..nodes.len())];
        let target = nodes[rng.gen_range(0..nodes.len())];
        
        if source != target {
            let _ = zipgraph_core::algorithms::dijkstra(graph, source, target);
        }
    }
    start.elapsed().as_secs_f64()
}

fn benchmark_betweenness_centrality(graph: &Graph) -> f64 {
    let start = Instant::now();
    let _ = betweenness_centrality(graph).unwrap();
    start.elapsed().as_secs_f64()
}

fn main() {
    println!("ZipGraph Performance Benchmark");
    println!("Rust implementation with parallel algorithms\n");

    let test_sizes = vec![
        (100, 200),
        (500, 1000),
        (1000, 2000),
        (5000, 10000),
    ];

    let mut results = BenchmarkSuite {
        library: "ZipGraph".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        benchmarks: Vec::new(),
    };

    for (n_nodes, n_edges) in test_sizes {
        println!("{}", "=".repeat(60));
        println!("Benchmarking with {} nodes, {} edges", n_nodes, n_edges);
        println!("{}", "=".repeat(60));

        // Graph creation
        let (create_time, graph) = benchmark_graph_creation(n_nodes, n_edges);
        println!("Graph creation: {:.6}s", create_time);

        // PageRank
        let pagerank_time = benchmark_pagerank(&graph);
        println!("PageRank: {:.6}s", pagerank_time);

        // Shortest paths
        let n_samples = (n_nodes / 10).min(100);
        let shortest_path_time = benchmark_shortest_paths(&graph, n_samples);
        println!("Shortest paths ({} samples): {:.6}s", n_samples, shortest_path_time);

        // Betweenness (skip for large graphs)
        let betweenness_time = if n_nodes <= 1000 {
            let time = benchmark_betweenness_centrality(&graph);
            println!("Betweenness centrality: {:.6}s", time);
            Some(time)
        } else {
            println!("Betweenness centrality: SKIPPED (optimized for smaller graphs)");
            None
        };

        results.benchmarks.push(BenchmarkResult {
            nodes: n_nodes,
            edges: n_edges,
            graph_creation_ms: create_time * 1000.0,
            pagerank_ms: pagerank_time * 1000.0,
            shortest_paths_ms: shortest_path_time * 1000.0,
            betweenness_ms: betweenness_time.map(|t| t * 1000.0),
        });
    }

    // Save results
    let json = serde_json::to_string_pretty(&results).unwrap();
    fs::write("benchmarks/zipgraph_benchmarks.json", json).unwrap();

    println!("\n{}", "=".repeat(60));
    println!("Results saved to benchmarks/zipgraph_benchmarks.json");
    println!("{}", "=".repeat(60));

    // Load and compare with NetworkX if available
    if let Ok(networkx_data) = fs::read_to_string("benchmarks/networkx_benchmarks.json") {
        if let Ok(networkx_results) = serde_json::from_str::<BenchmarkSuite>(&networkx_data) {
            println!("\n{}", "=".repeat(60));
            println!("PERFORMANCE COMPARISON: ZipGraph vs NetworkX");
            println!("{}", "=".repeat(60));
            
            for (zipgraph, networkx) in results.benchmarks.iter().zip(networkx_results.benchmarks.iter()) {
                println!("\nGraph: {} nodes, {} edges", zipgraph.nodes, zipgraph.edges);
                println!("  Graph Creation: {:.2}x faster", 
                    networkx.graph_creation_ms / zipgraph.graph_creation_ms);
                println!("  PageRank:       {:.2}x faster", 
                    networkx.pagerank_ms / zipgraph.pagerank_ms);
                println!("  Shortest Paths: {:.2}x faster", 
                    networkx.shortest_paths_ms / zipgraph.shortest_paths_ms);
                
                if let (Some(zg_bet), Some(nx_bet)) = (zipgraph.betweenness_ms, networkx.betweenness_ms) {
                    println!("  Betweenness:    {:.2}x faster", nx_bet / zg_bet);
                }
            }
        }
    }
}
