mod data;
mod graph;
mod analysis;

fn main() {
    let filepath = "finalproject.csv";
    let participants = data::load_data(filepath).expect("Failed to load data");

    // Create and build the graph
    let mut graph = graph::create_graph(&participants);

    graph.connect_on_threshold(0.5); 

    // Perform analysis
    analysis::analyze_graph(&graph);
    analysis::analyze_depression_by_age(&participants);
}

