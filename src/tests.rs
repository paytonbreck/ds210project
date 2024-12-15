#[cfg(test)]
mod tests {
    use crate::graph::{build_graph, NodeData};
    use crate::utility::cosine_similarity;
    use petgraph::graph::{Graph, NodeIndex};
    use std::collections::HashMap;

    #[test]
    fn test_cosine_similarity() {
        let vec1 = vec![1.0, 0.0, 1.0];
        let vec2 = vec![0.0, 1.0, 1.0];
        let similarity = cosine_similarity(&vec1, &vec2);
        assert!(similarity > 0.5 && similarity < 0.8, "Unexpected similarity value");
    }

    #[test]
    fn test_add_nodes_and_edges() {
        let file_path = "Seats held by women in Parliament.csv"; 
        let (graph, node_indices) = build_graph(file_path).expect("Failed to build graph");
        assert!(!node_indices.is_empty(), "Graph has no nodes");
        assert!(graph.edge_count() > 0, "Graph has no edges");
    }

    #[test]
    fn test_calculate_distance() {
        let mut graph = Graph::<NodeData, f64>::new();
        let node1 = graph.add_node(NodeData {
            region: "CountryA".to_string(),
            year: 2015,
            percentage: 20.0,
        });
        let node2 = graph.add_node(NodeData {
            region: "CountryB".to_string(),
            year: 2015,
            percentage: 30.0,
        });
        let node3 = graph.add_node(NodeData {
            region: "CountryC".to_string(),
            year: 2015,
            percentage: 25.0,
        });
        graph.add_edge(node1, node2, 0.9);
        graph.add_edge(node2, node3, 0.8);
        let distance = crate::graph::calculate_distance(&graph, node1, node3);
        assert_eq!(distance, Some(2), "Unexpected distance between nodes");
    }

    #[test]
    fn test_average_distance() {
        let mut graph = Graph::<NodeData, f64>::new();
        let node1 = graph.add_node(NodeData {
            region: "CountryA".to_string(),
            year: 2015,
            percentage: 20.0,
        });
        let node2 = graph.add_node(NodeData {
            region: "CountryB".to_string(),
            year: 2015,
            percentage: 30.0,
        });
        let node3 = graph.add_node(NodeData {
            region: "CountryC".to_string(),
            year: 2015,
            percentage: 25.0,
        });
        graph.add_edge(node1, node2, 0.9);
        graph.add_edge(node2, node3, 0.8);
        graph.add_edge(node1, node3, 0.7);
        let avg_distance = crate::graph::average_distance(&graph);
        assert!(avg_distance.is_some(), "Average distance calculation failed");
        assert!(avg_distance.unwrap() > 0.0, "Unexpected average distance value");
    }
}