#!/usr/bin/env python3
"""
NetworkX Performance Baseline
Compare ZipGraph against Python NetworkX for common graph operations
"""

import networkx as nx
import time
import json
import random

def benchmark_graph_creation(n_nodes, n_edges):
    """Benchmark graph creation time"""
    start = time.time()
    G = nx.Graph()
    
    # Add nodes
    for i in range(n_nodes):
        G.add_node(i, label=f"Node{i}")
    
    # Add random edges
    edges_added = 0
    while edges_added < n_edges:
        u = random.randint(0, n_nodes - 1)
        v = random.randint(0, n_nodes - 1)
        if u != v and not G.has_edge(u, v):
            G.add_edge(u, v, weight=random.random())
            edges_added += 1
    
    elapsed = time.time() - start
    return elapsed, G

def benchmark_pagerank(G, max_iter=100):
    """Benchmark PageRank computation"""
    start = time.time()
    pagerank = nx.pagerank(G, max_iter=max_iter, tol=1e-6)
    elapsed = time.time() - start
    return elapsed, pagerank

def benchmark_shortest_paths(G, n_samples=100):
    """Benchmark shortest path computations"""
    nodes = list(G.nodes())
    if len(nodes) < 2:
        return 0
    
    start = time.time()
    for _ in range(min(n_samples, len(nodes) // 2)):
        source = random.choice(nodes)
        target = random.choice(nodes)
        if source != target:
            try:
                path = nx.shortest_path(G, source, target, weight='weight')
            except nx.NetworkXNoPath:
                pass
    elapsed = time.time() - start
    return elapsed

def benchmark_betweenness_centrality(G):
    """Benchmark betweenness centrality"""
    start = time.time()
    betweenness = nx.betweenness_centrality(G)
    elapsed = time.time() - start
    return elapsed, betweenness

def run_benchmarks():
    """Run all benchmarks and save results"""
    results = {
        "library": "NetworkX",
        "version": nx.__version__,
        "benchmarks": []
    }
    
    test_sizes = [
        (100, 200),
        (500, 1000),
        (1000, 2000),
        (5000, 10000),
    ]
    
    for n_nodes, n_edges in test_sizes:
        print(f"\n{'='*60}")
        print(f"Benchmarking with {n_nodes} nodes, {n_edges} edges")
        print(f"{'='*60}")
        
        # Graph creation
        create_time, G = benchmark_graph_creation(n_nodes, n_edges)
        print(f"Graph creation: {create_time:.4f}s")
        
        # PageRank
        pagerank_time, _ = benchmark_pagerank(G)
        print(f"PageRank: {pagerank_time:.4f}s")
        
        # Shortest paths
        shortest_path_time = benchmark_shortest_paths(G, n_samples=min(100, n_nodes // 10))
        print(f"Shortest paths (100 samples): {shortest_path_time:.4f}s")
        
        # Betweenness (skip for large graphs - too slow)
        if n_nodes <= 1000:
            betweenness_time, _ = benchmark_betweenness_centrality(G)
            print(f"Betweenness centrality: {betweenness_time:.4f}s")
        else:
            betweenness_time = None
            print(f"Betweenness centrality: SKIPPED (too slow for {n_nodes} nodes)")
        
        results["benchmarks"].append({
            "nodes": n_nodes,
            "edges": n_edges,
            "graph_creation_ms": create_time * 1000,
            "pagerank_ms": pagerank_time * 1000,
            "shortest_paths_ms": shortest_path_time * 1000,
            "betweenness_ms": betweenness_time * 1000 if betweenness_time else None,
        })
    
    # Save results
    with open("networkx_benchmarks.json", "w") as f:
        json.dump(results, f, indent=2)
    
    print(f"\n{'='*60}")
    print("Results saved to networkx_benchmarks.json")
    print(f"{'='*60}")

if __name__ == "__main__":
    print("NetworkX Performance Baseline")
    print("Python implementation of common graph algorithms")
    run_benchmarks()
