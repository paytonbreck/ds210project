use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};
use plotters::backend::BitMapBackend;
use plotters::prelude::{WHITE, BLUE, RED, EmptyElement, Circle, ChartBuilder, PointSeries, Text};
use plotters::style::IntoFont;
use plotters::drawing::IntoDrawingArea;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct NodeData {
    pub region: String,
    pub year: i32,
    pub percentage: f64,
}

pub fn build_graph(file_path: &str) -> Result<(Graph<NodeData, f64>, HashMap<String, NodeIndex>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let mut graph = Graph::<NodeData, f64>::new();
    let mut nodes = HashMap::new();
    let mut year_data: HashMap<String, Vec<f64>> = HashMap::new();
    for result in reader.records() {
        let record = result?;
        let country = record[1].to_string();
        let value: f64 = record[6].parse().unwrap_or_else(|_| {
            eprintln!("Skipping invalid value for {}: '{}'", country, record[6].to_string());
            0.0
        });
        year_data.entry(country.clone()).or_insert_with(Vec::new).push(value);
        if !nodes.contains_key(&country) {
            let node = graph.add_node(NodeData {
                region: country.clone(),
                year: record[2].parse().unwrap_or(0),
                percentage: value,
            });
            nodes.insert(country, node);
        }
    }
    for (country1, data1) in &year_data {
        for (country2, data2) in &year_data {
            if country1 != country2 {
                let similarity = crate::utility::cosine_similarity(data1, data2);
                if similarity > 0.8 {
                    graph.add_edge(nodes[country1], nodes[country2], similarity);
                }
            }
        }
    }
    Ok((graph, nodes))
}

pub fn visualize_graph(
    graph: &Graph<NodeData, f64>,
    node_to_cluster: &HashMap<NodeIndex, usize>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut dot_file = File::create("graph.dot")?;
    let dot_format = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    write!(dot_file, "{:?}", dot_format)?;
    let backend = BitMapBackend::new(file_path, (800, 600));
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Women's Representation in National Parliaments", ("sans-serif", 20).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(2000..2025, 0..100)?;
    chart.configure_mesh()
        .x_desc("Year")
        .y_desc("Percentage of Seats Held by Women")
        .draw()?;
    for node in graph.node_indices() {
        let data = &graph[node];
        let cluster_id = node_to_cluster.get(&node).unwrap_or(&0);
        let x = data.year;
        let y = data.percentage;
        let color = match cluster_id {
            0 => BLUE,
            _ => RED,
        };
        chart.draw_series(PointSeries::of_element(
            vec![(x as i32, y as i32)],
            5,
            &color,
            &|coord, size, color| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, color.filled())
                    + Text::new(
                        format!("{} ({:.1}%)", data.region, y),
                        (0, 10),
                        ("sans-serif", 10),
                    )
            },
        ))?; 
    }
    Ok(())
}