//! Example: Basic graph operations and shortest path finding

use zipgraph_core::{algorithms, Graph};
use zipgraph_optimizer::QueryOptimizer;

fn main() {
    println!("🚀 ZipGraph - Basic Usage Example\n");

    // Create a simple graph
    let mut graph = Graph::new();
    
    println!("Creating nodes...");
    let city_a = graph.add_node_simple("City A");
    let city_b = graph.add_node_simple("City B");
    let city_c = graph.add_node_simple("City C");
    let city_d = graph.add_node_simple("City D");
    let city_e = graph.add_node_simple("City E");

    println!("Adding roads (edges)...");
    graph.add_edge(city_a, city_b, 4.0).unwrap(); // A -> B: 4km
    graph.add_edge(city_a, city_c, 2.0).unwrap(); // A -> C: 2km
    graph.add_edge(city_b, city_c, 1.0).unwrap(); // B -> C: 1km
    graph.add_edge(city_b, city_d, 5.0).unwrap(); // B -> D: 5km
    graph.add_edge(city_c, city_d, 8.0).unwrap(); // C -> D: 8km
    graph.add_edge(city_c, city_e, 10.0).unwrap(); // C -> E: 10km
    graph.add_edge(city_d, city_e, 2.0).unwrap(); // D -> E: 2km

    println!("\n📊 Graph Statistics:");
    println!("  Nodes: {}", graph.node_count());
    println!("  Edges: {}", graph.edge_count());

    // Find shortest path using different algorithms
    println!("\n🔍 Finding shortest path from City A to City E...\n");

    // BFS (unweighted)
    println!("1. Breadth-First Search (BFS):");
    match algorithms::bfs(&graph, city_a, city_e) {
        Ok(path) => {
            print!("   Path: ");
            for (i, node) in path.iter().enumerate() {
                print!("City {} -> ", (65 + node) as u8 as char);
                if i == path.len() - 1 {
                    println!("\n   Hops: {}", path.len() - 1);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Dijkstra (weighted)
    println!("\n2. Dijkstra's Algorithm (weighted):");
    match algorithms::dijkstra(&graph, city_a, city_e) {
        Ok((path, cost)) => {
            print!("   Path: ");
            for node in &path {
                print!("City {} -> ", (65 + node) as u8 as char);
            }
            println!("\n   Distance: {:.1}km", cost);
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Using optimizer with caching
    println!("\n3. Query Optimizer (with ML + caching):");
    let mut optimizer = QueryOptimizer::new();
    
    match optimizer.shortest_path(&graph, city_a, city_e) {
        Ok(path) => {
            print!("   Path: ");
            for node in &path {
                print!("City {} -> ", (65 + node) as u8 as char);
            }
            println!();
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Query again to show caching
    println!("\n   Running same query again (should hit cache)...");
    optimizer.shortest_path(&graph, city_a, city_e).unwrap();
    
    println!("\n📈 Optimizer Stats:");
    println!("   {}", optimizer.stats());

    // Explore neighbors
    println!("\n🌐 Neighbors of City C:");
    match graph.neighbors(city_c) {
        Ok(neighbors) => {
            for neighbor in neighbors {
                println!("   -> City {}", (65 + neighbor) as u8 as char);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n✅ Example complete!");
}
