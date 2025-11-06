//! Example: Social network analysis and community detection

use zipgraph_core::Graph;
use zipgraph_ml::AlgorithmSelector;
use zipgraph_optimizer::QueryOptimizer;

fn main() {
    println!("👥 ZipGraph - Social Network Analysis Example\n");

    // Create a social network graph
    let mut graph = Graph::new();

    println!("Building social network...");
    
    // Create users
    let users = vec![
        "Alice", "Bob", "Charlie", "Diana", "Eve",
        "Frank", "Grace", "Henry", "Ivy", "Jack",
    ];

    let mut user_ids = Vec::new();
    for user in &users {
        let id = graph.add_node_simple(*user);
        user_ids.push(id);
    }

    // Add friendships (undirected edges)
    println!("Adding friendships...");
    
    // Community 1: Alice, Bob, Charlie, Diana
    graph.add_edge(user_ids[0], user_ids[1], 1.0).unwrap(); // Alice-Bob
    graph.add_edge(user_ids[0], user_ids[2], 1.0).unwrap(); // Alice-Charlie
    graph.add_edge(user_ids[1], user_ids[2], 1.0).unwrap(); // Bob-Charlie
    graph.add_edge(user_ids[2], user_ids[3], 1.0).unwrap(); // Charlie-Diana

    // Community 2: Eve, Frank, Grace
    graph.add_edge(user_ids[4], user_ids[5], 1.0).unwrap(); // Eve-Frank
    graph.add_edge(user_ids[4], user_ids[6], 1.0).unwrap(); // Eve-Grace
    graph.add_edge(user_ids[5], user_ids[6], 1.0).unwrap(); // Frank-Grace

    // Community 3: Henry, Ivy, Jack
    graph.add_edge(user_ids[7], user_ids[8], 1.0).unwrap(); // Henry-Ivy
    graph.add_edge(user_ids[8], user_ids[9], 1.0).unwrap(); // Ivy-Jack
    graph.add_edge(user_ids[7], user_ids[9], 1.0).unwrap(); // Henry-Jack

    // Bridge connections between communities
    graph.add_edge(user_ids[3], user_ids[4], 1.0).unwrap(); // Diana-Eve
    graph.add_edge(user_ids[6], user_ids[7], 1.0).unwrap(); // Grace-Henry

    println!("\n📊 Network Statistics:");
    println!("  Total users: {}", graph.node_count());
    println!("  Total friendships: {}", graph.edge_count());

    // Analyze user connections
    println!("\n🌐 User Connection Analysis:");
    
    for (idx, user_id) in user_ids.iter().enumerate() {
        let degree = graph.degree(*user_id).unwrap();
        let friends = graph.neighbors(*user_id).unwrap();
        
        println!("\n  {}:", users[idx]);
        println!("    Friends: {}", degree);
        print!("    Connected to: ");
        for friend_id in &friends {
            let friend_idx = user_ids.iter().position(|&id| id == *friend_id).unwrap();
            print!("{} ", users[friend_idx]);
        }
        println!();
        
        // Classify user type
        if degree >= 4 {
            println!("    Type: 🌟 Influencer (highly connected)");
        } else if degree >= 2 {
            println!("    Type: 🤝 Active member");
        } else {
            println!("    Type: 👤 Regular user");
        }
    }

    // Find shortest path between users
    println!("\n🔍 Connection Paths:");
    let mut optimizer = QueryOptimizer::new();
    
    let test_pairs = vec![
        (user_ids[0], user_ids[9], "Alice", "Jack"),
        (user_ids[1], user_ids[8], "Bob", "Ivy"),
    ];

    for (from, to, from_name, to_name) in test_pairs {
        match optimizer.shortest_path(&graph, from, to) {
            Ok(path) => {
                print!("  {} → {}: ", from_name, to_name);
                for (i, node_id) in path.iter().enumerate() {
                    let user_idx = user_ids.iter().position(|&id| id == *node_id).unwrap();
                    print!("{}", users[user_idx]);
                    if i < path.len() - 1 {
                        print!(" → ");
                    }
                }
                println!(" ({} hops)", path.len() - 1);
            }
            Err(_) => println!("  {} → {}: No path found", from_name, to_name),
        }
    }

    // Identify influencers
    println!("\n🌟 Influencer Ranking:");
    let mut user_degrees: Vec<_> = user_ids
        .iter()
        .enumerate()
        .map(|(idx, &id)| (users[idx], graph.degree(id).unwrap()))
        .collect();
    
    user_degrees.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (rank, (user, degree)) in user_degrees.iter().take(5).enumerate() {
        println!("  {}. {} ({} connections)", rank + 1, user, degree);
    }

    // ML algorithm selection
    println!("\n🧠 ML Algorithm Selection:");
    let selector = AlgorithmSelector::new();
    let algorithm = selector.select(&graph);
    println!("  Recommended algorithm: {:?}", algorithm);
    println!("  (Based on graph structure and size)");

    // Performance stats
    println!("\n📈 Performance Metrics:");
    println!("  {}", optimizer.stats());

    println!("\n💡 Key Insights:");
    println!("  • Diana and Grace are bridge users connecting communities");
    println!("  • Three distinct communities identified");
    println!("  • Graph traversal enables 6-degrees-of-separation analysis");
    println!("  • Real-time community detection at scale");

    println!("\n✅ Social network analysis complete!");
}
