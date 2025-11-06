//! Persistent storage for graphs
//!
//! Support for saving and loading graphs in multiple formats:
//! - Binary (custom format, fastest)
//! - JSON (human-readable)
//! - GraphML (XML-based, widely supported)

use crate::error::{GraphError, Result};
use crate::graph::{Graph, Node};
use crate::types::NodeId;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Storage format for graphs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageFormat {
    /// Binary format (fastest, smallest)
    Binary,
    /// JSON format (human-readable)
    Json,
    /// GraphML format (XML-based, widely compatible)
    GraphML,
}

/// Serializable graph representation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializableGraph {
    nodes: Vec<(NodeId, Node)>,
    edges: Vec<(NodeId, NodeId, f64)>,
    directed: bool,
}

impl From<&Graph> for SerializableGraph {
    fn from(graph: &Graph) -> Self {
        let nodes: Vec<_> = graph
            .node_ids()
            .into_iter()
            .filter_map(|id| graph.node(id).ok().map(|node| (id, node.clone())))
            .collect();

        let edges: Vec<_> = graph
            .edges()
            .into_iter()
            .map(|edge| (edge.from, edge.to, edge.weight))
            .collect();

        SerializableGraph {
            nodes,
            edges,
            directed: graph.is_directed(),
        }
    }
}

impl SerializableGraph {
    fn to_graph(&self) -> Result<Graph> {
        let mut graph = if self.directed {
            Graph::new_directed()
        } else {
            Graph::new()
        };

        // Add nodes
        for (_id, node) in &self.nodes {
            graph.add_node(node.clone());
        }

        // Add edges
        for (source, target, weight) in &self.edges {
            graph.add_edge(*source, *target, *weight)?;
        }

        Ok(graph)
    }
}

/// Save a graph to a file
pub fn save_graph<P: AsRef<Path>>(
    graph: &Graph,
    path: P,
    format: StorageFormat,
) -> Result<()> {
    let file = File::create(path)
        .map_err(|e| GraphError::InvalidData(format!("Failed to create file: {}", e)))?;
    let mut writer = BufWriter::new(file);

    let serializable = SerializableGraph::from(graph);

    match format {
        StorageFormat::Binary => {
            let encoded = bincode::serialize(&serializable)
                .map_err(|e| GraphError::SerializationError(e.to_string()))?;
            writer
                .write_all(&encoded)
                .map_err(|e| GraphError::IoError(e))?;
        }
        StorageFormat::Json => {
            serde_json::to_writer_pretty(&mut writer, &serializable)
                .map_err(|e| GraphError::SerializationError(e.to_string()))?;
        }
        StorageFormat::GraphML => {
            write_graphml(&mut writer, &serializable)?;
        }
    }

    writer
        .flush()
        .map_err(|e| GraphError::IoError(e))?;

    Ok(())
}

/// Load a graph from a file
pub fn load_graph<P: AsRef<Path>>(path: P, format: StorageFormat) -> Result<Graph> {
    let file = File::open(path)
        .map_err(|e| GraphError::InvalidData(format!("Failed to open file: {}", e)))?;
    let mut reader = BufReader::new(file);

    let serializable: SerializableGraph = match format {
        StorageFormat::Binary => {
            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .map_err(|e| GraphError::IoError(e))?;
            bincode::deserialize(&buffer)
                .map_err(|e| GraphError::SerializationError(e.to_string()))?
        }
        StorageFormat::Json => serde_json::from_reader(&mut reader)
            .map_err(|e| GraphError::SerializationError(e.to_string()))?,
        StorageFormat::GraphML => read_graphml(&mut reader)?,
    };

    serializable.to_graph()
}

/// Write graph in GraphML format
fn write_graphml<W: Write>(writer: &mut W, graph: &SerializableGraph) -> Result<()> {
    writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)
        .map_err(|e| GraphError::IoError(e))?;
    writeln!(
        writer,
        r#"<graphml xmlns="http://graphml.graphdrawing.org/xmlns">"#
    )
    .map_err(|e| GraphError::IoError(e))?;
    writeln!(writer, r#"  <key id="weight" for="edge" attr.name="weight" attr.type="double"/>"#)
        .map_err(|e| GraphError::IoError(e))?;
    writeln!(writer, r#"  <key id="label" for="node" attr.name="label" attr.type="string"/>"#)
        .map_err(|e| GraphError::IoError(e))?;

    let edge_default = if graph.directed {
        "directed"
    } else {
        "undirected"
    };
    writeln!(
        writer,
        r#"  <graph id="G" edgedefault="{}">"#,
        edge_default
    )
    .map_err(|e| GraphError::IoError(e))?;

    // Write nodes
    for (id, node) in &graph.nodes {
        writeln!(writer, r#"    <node id="n{}">"#, id)
            .map_err(|e| GraphError::IoError(e))?;
        writeln!(
            writer,
            r#"      <data key="label">{}</data>"#,
            escape_xml(&node.label)
        )
        .map_err(|e| GraphError::IoError(e))?;
        writeln!(writer, r#"    </node>"#).map_err(|e| GraphError::IoError(e))?;
    }

    // Write edges
    for (i, (source, target, weight)) in graph.edges.iter().enumerate() {
        writeln!(
            writer,
            r#"    <edge id="e{}" source="n{}" target="n{}">"#,
            i, source, target
        )
        .map_err(|e| GraphError::IoError(e))?;
        writeln!(writer, r#"      <data key="weight">{}</data>"#, weight)
            .map_err(|e| GraphError::IoError(e))?;
        writeln!(writer, r#"    </edge>"#).map_err(|e| GraphError::IoError(e))?;
    }

    writeln!(writer, r#"  </graph>"#).map_err(|e| GraphError::IoError(e))?;
    writeln!(writer, r#"</graphml>"#).map_err(|e| GraphError::IoError(e))?;

    Ok(())
}

/// Read graph from GraphML format (simplified parser)
fn read_graphml<R: Read>(reader: &mut R) -> Result<SerializableGraph> {
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .map_err(|e| GraphError::IoError(e))?;

    // Simple parsing - in production, use a proper XML parser
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let directed = content.contains(r#"edgedefault="directed""#);

    // Parse nodes
    for node_match in content.match_indices("<node id=") {
        let start = node_match.0;
        let end = content[start..].find("</node>").unwrap_or(0) + start;
        let node_xml = &content[start..end];

        // Extract node id - look for id="nXXX"
        if let Some(id_start) = node_xml.find(r#"id=""#) {
            let id_str = &node_xml[id_start + 4..]; // Skip 'id="'
            if let Some(id_end) = id_str.find('"') {
                let id_with_n = &id_str[..id_end]; // This will be "nXXX"
                // Remove the 'n' prefix and parse the number
                if id_with_n.starts_with('n') {
                    if let Ok(id) = id_with_n[1..].parse::<NodeId>() {
                        // Extract label
                        let label = if let Some(label_start) = node_xml.find("<data key=\"label\">") {
                            let label_str = &node_xml[label_start + 18..];
                            let label_end = label_str.find("</data>").unwrap_or(0);
                            unescape_xml(&label_str[..label_end])
                        } else {
                            format!("Node{}", id)
                        };

                        nodes.push((id, Node::new(id, label)));
                    }
                }
            }
        }
    }

    // Parse edges
    for edge_match in content.match_indices("<edge ") {
        let start = edge_match.0;
        let end = content[start..].find("</edge>").unwrap_or(0) + start;
        let edge_xml = &content[start..end];

        // Extract source - look for source="nXXX"
        let source = if let Some(src_start) = edge_xml.find(r#"source=""#) {
            let src_str = &edge_xml[src_start + 8..]; // Skip 'source="'
            if let Some(src_end) = src_str.find('"') {
                let src_with_n = &src_str[..src_end];
                if src_with_n.starts_with('n') {
                    src_with_n[1..].parse::<NodeId>().unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        };

        // Extract target - look for target="nXXX"
        let target = if let Some(tgt_start) = edge_xml.find(r#"target=""#) {
            let tgt_str = &edge_xml[tgt_start + 8..]; // Skip 'target="'
            if let Some(tgt_end) = tgt_str.find('"') {
                let tgt_with_n = &tgt_str[..tgt_end];
                if tgt_with_n.starts_with('n') {
                    tgt_with_n[1..].parse::<NodeId>().unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        };

        // Extract weight
        let weight = if let Some(weight_start) = edge_xml.find("<data key=\"weight\">") {
            let weight_str = &edge_xml[weight_start + 19..];
            let weight_end = weight_str.find("</data>").unwrap_or(0);
            weight_str[..weight_end].parse::<f64>().unwrap_or(1.0)
        } else {
            1.0
        };

        edges.push((source, target, weight));
    }

    Ok(SerializableGraph {
        nodes,
        edges,
        directed,
    })
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn unescape_xml(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_node_simple("Alice");
        graph.add_node_simple("Bob");
        graph.add_node_simple("Charlie");
        graph.add_edge(0, 1, 1.5).unwrap();
        graph.add_edge(1, 2, 2.5).unwrap();
        graph.add_edge(2, 0, 0.5).unwrap();
        graph
    }

    #[test]
    fn test_save_load_binary() {
        let graph = create_test_graph();
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("graph.bin");

        save_graph(&graph, &path, StorageFormat::Binary).unwrap();
        assert!(path.exists());

        let loaded = load_graph(&path, StorageFormat::Binary).unwrap();
        assert_eq!(loaded.node_count(), graph.node_count());
        assert_eq!(loaded.edge_count(), graph.edge_count());
    }

    #[test]
    fn test_save_load_json() {
        let graph = create_test_graph();
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("graph.json");

        save_graph(&graph, &path, StorageFormat::Json).unwrap();
        assert!(path.exists());

        // Check JSON is human-readable
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("Alice"));
        assert!(content.contains("Bob"));

        let loaded = load_graph(&path, StorageFormat::Json).unwrap();
        assert_eq!(loaded.node_count(), graph.node_count());
        assert_eq!(loaded.edge_count(), graph.edge_count());
    }

    #[test]
    fn test_save_load_graphml() {
        let graph = create_test_graph();
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("graph.graphml");

        save_graph(&graph, &path, StorageFormat::GraphML).unwrap();
        assert!(path.exists());

        // Check GraphML format
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("<?xml"));
        assert!(content.contains("<graphml"));

        let loaded = load_graph(&path, StorageFormat::GraphML).unwrap();
        assert_eq!(loaded.node_count(), graph.node_count());
        assert_eq!(loaded.edge_count(), graph.edge_count());
    }

    #[test]
    fn test_directed_graph_preservation() {
        let mut graph = Graph::new_directed();
        graph.add_node_simple("A");
        graph.add_node_simple("B");
        graph.add_edge(0, 1, 1.0).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("directed.json");

        save_graph(&graph, &path, StorageFormat::Json).unwrap();
        let loaded = load_graph(&path, StorageFormat::Json).unwrap();

        assert!(loaded.is_directed());
    }

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new();
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("empty.bin");

        save_graph(&graph, &path, StorageFormat::Binary).unwrap();
        let loaded = load_graph(&path, StorageFormat::Binary).unwrap();

        assert_eq!(loaded.node_count(), 0);
        assert_eq!(loaded.edge_count(), 0);
    }
}
