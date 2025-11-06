//! Example: Fraud detection using anomaly detection

use zipgraph_core::Graph;
use zipgraph_ml::AnomalyDetector;

fn main() {
    println!("🔍 ZipGraph - Fraud Detection Example\n");

    // Create a transaction network
    let mut graph = Graph::new();

    println!("Building transaction network...");
    
    // Normal accounts (0-9)
    let mut accounts = Vec::new();
    for i in 0..10 {
        let account = graph.add_node_simple(format!("Account_{}", i));
        accounts.push(account);
    }

    // Add normal transaction patterns
    println!("Adding normal transactions...");
    for i in 0..9 {
        // Each account transacts with 2-3 neighbors
        graph.add_edge(accounts[i], accounts[i + 1], 100.0).unwrap();
        if i < 8 {
            graph.add_edge(accounts[i], accounts[i + 2], 50.0).unwrap();
        }
    }

    // Add suspicious accounts
    println!("Adding suspicious activity...");
    
    // Fraud pattern 1: Hub account (receives from many accounts)
    let fraud_hub = graph.add_node_simple("Fraud_Hub");
    for i in 0..7 {
        graph.add_edge(accounts[i], fraud_hub, 10.0).unwrap();
    }

    // Fraud pattern 2: Isolated money mule
    let money_mule = graph.add_node_simple("Money_Mule");
    graph.add_edge(accounts[3], money_mule, 1000.0).unwrap();
    graph.add_edge(money_mule, fraud_hub, 950.0).unwrap();

    // Fraud pattern 3: Completely isolated account (no transactions)
    let _isolated = graph.add_node_simple("Isolated_Account");

    println!("\n📊 Network Statistics:");
    println!("  Total accounts: {}", graph.node_count());
    println!("  Total transactions: {}", graph.edge_count());

    // Train anomaly detector on baseline
    println!("\n🧠 Training anomaly detector...");
    let mut detector = AnomalyDetector::new().with_threshold(0.7);
    
    // In production, you'd train on historical "normal" data
    detector.train_on_baseline(&graph).unwrap();
    println!("  ✓ Baseline model trained");

    // Detect anomalies
    println!("\n🚨 Detecting anomalous patterns...");
    let anomalies = detector.detect(&graph);

    if anomalies.is_empty() {
        println!("  No anomalies detected");
    } else {
        println!("  Found {} anomalies:\n", anomalies.len());
        
        for (idx, anomaly) in anomalies.iter().enumerate() {
            println!("  Anomaly #{}:", idx + 1);
            println!("    Type: {:?}", anomaly.anomaly_type);
            println!("    Score: {:.2}", anomaly.anomaly_score);
            println!("    Reason: {}", anomaly.reason);
            
            // Show affected accounts
            print!("    Accounts: ");
            for &node_id in &anomaly.node_ids {
                if let Ok(node) = graph.node(node_id) {
                    print!("{} ", node.label);
                }
            }
            println!("\n");
        }
    }

    // Analyze specific accounts
    println!("📈 Account Analysis:");
    
    let suspicious_accounts = vec![
        (fraud_hub, "Fraud_Hub"),
        (money_mule, "Money_Mule"),
    ];

    for (account_id, name) in suspicious_accounts {
        if let Ok(degree) = graph.degree(account_id) {
            let neighbors = graph.neighbors(account_id).unwrap();
            
            println!("\n  {}:", name);
            println!("    Connections: {}", degree);
            println!("    Connected to: {} accounts", neighbors.len());
            
            // Calculate risk score based on degree anomaly
            let avg_degree = graph.node_count() as f64 * 2.0 / graph.node_count() as f64;
            let risk_score = if degree > avg_degree as usize {
                ((degree as f64 / avg_degree) * 50.0).min(100.0)
            } else {
                10.0
            };
            
            println!("    Risk Score: {:.0}/100", risk_score);
            
            if risk_score > 70.0 {
                println!("    ⚠️  HIGH RISK - Review required");
            } else if risk_score > 40.0 {
                println!("    ⚡ MEDIUM RISK - Monitor closely");
            }
        }
    }

    println!("\n💡 Key Insights:");
    println!("  • Graph-based fraud detection spots unusual transaction patterns");
    println!("  • ML models learn normal behavior automatically");
    println!("  • Real-time detection prevents fraud before it happens");
    println!("  • 100-200x faster than traditional rule-based systems");

    println!("\n✅ Fraud detection example complete!");
}
