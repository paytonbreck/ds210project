# ds210project
This is my project for DS210: Programming for Data Science at Boston University

Overview

This project explores the relationship between women's representation in national parliaments and other factors such as geographical region and time. The project involves creating a graph representation of the dataset, clustering similar regions based on their data, and visualizing trends in women's parliamentary representation over time. A major focus is on identifying patterns and relationships among countries and regions using graph theory and data visualization techniques.

Dataset

The dataset used for this project is based on data reflecting the percentage of parliamentary seats held by women across different countries over time. It includes data such as:
Region: The country or geographical region.
Year: The year of observation.
Percentage: The percentage of parliamentary seats held by women in that year.

The dataset is sourced from https://data.un.org/

Implementation

Graph Construction

The project constructs a graph where nodes represent individual countries or regions and edges represent the similarity between regions based on trends in women's representation over the years. The similarity metric is calculated using cosine similarity on yearly percentage data. The graph allows for analyzing relationships and clustering countries with similar patterns in women's representation.

Clustering

The detect_clusters function identifies clusters of countries with similar trends using a breadth-first search (BFS) approach. Each cluster groups countries that exhibit comparable patterns, enabling meaningful comparisons between regions.

Distance Calculations

Two functions are implemented to analyze the graph structure:
calculate_distance: Uses BFS to determine the shortest path between two nodes, which in this case are countries.
average_distance: Computes the average shortest path between all pairs of nodes in the graph.

Testing

Tests are implemented to ensure the correctness of: adding nodes and edges to the graph, calculating distances between nodes, and verifying the clustering mechanism.

Visualization

The project visualizes clusters of regions. The visualization highlights clusters of similar regions and are implemented using the plotters library.

Future Directions

Potential extensions to the project includes incorporating additional demographic or socioeconomic data and expanding the dataset to include more years or regions.
