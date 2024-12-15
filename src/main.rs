mod graph;
mod utility;
use crate::graph::{build_graph, visualize_graph};
use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashSet, HashMap};

fn main() {
    let input_file = "Seats held by women in Parliament.csv";
    let (graph, _node_indices) = build_graph(input_file).expect("Failed to build graph");
    let num_nodes = graph.node_count();
    let num_edges = graph.edge_count();
    println!("Graph constructed with {} nodes and {} edges.", num_nodes, num_edges);
    let (clusters, node_to_cluster) = detect_clusters(&graph);
    println!("Clusters of similar countries:");
    for (i, cluster) in clusters.iter().enumerate() {
        let cluster_names: Vec<_> = cluster
            .iter()
            .map(|&node| &graph[node].region)
            .collect();
        println!("Cluster {}: {:?}", i + 1, cluster_names);
    }
    if let Err(e) = visualize_graph(&graph, &node_to_cluster, "graph_visualization.png") {
        eprintln!("Visualization error: {}", e);
    } else {
        println!("Graph visualization saved as 'graph_visualization.png' and 'graph.dot'.");
    }
}

pub fn detect_clusters(
    graph: &Graph<graph::NodeData, f64>,
) -> (Vec<Vec<NodeIndex>>, HashMap<NodeIndex, usize>) {
    let mut visited = HashSet::new();
    let mut clusters = Vec::new();
    let mut node_to_cluster: HashMap<NodeIndex, usize> = HashMap::new();
    for node in graph.node_indices() {
        if !visited.contains(&node) {
            let mut cluster = Vec::new();
            let mut stack = vec![node];
            let cluster_id = clusters.len();
            while let Some(current) = stack.pop() {
                if visited.insert(current) {
                    cluster.push(current);
                    node_to_cluster.insert(current, cluster_id);
                    for neighbor in graph.neighbors(current) {
                        if !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
            clusters.push(cluster);
        }
    }
    (clusters, node_to_cluster)
}