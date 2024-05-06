// Remove the following imports if they are not needed
use petgraph::Graph;
use crate::data::Participant;
use petgraph::graph::NodeIndex; 
type UnGraph<N, E, Ix = petgraph::graph::DefaultIx> = Graph<N, E, petgraph::Undirected, Ix>; // Define UnGraph as an alias


pub struct MentalHealthGraph {
    pub graph: UnGraph<Participant, f32>, // Using an undirected graph with Participant nodes and f32 edge weights.
}

impl MentalHealthGraph {
    // Creates a new empty `MentalHealthGraph`.
    pub fn new() -> Self {
        MentalHealthGraph {
            graph: Graph::new_undirected(),
        }
    }

    // Adds a participant to the graph and returns the index of the new node.
    pub fn add_participant(&mut self, participant: Participant) -> NodeIndex {
        self.graph.add_node(participant)
    }

    // Adds an edge between two participants with a specified weight if the edge does not already exist.
    pub fn add_or_update_edge(&mut self, a: NodeIndex, b: NodeIndex, weight: f32) {
        if let Some(_edge_id) = self.graph.find_edge(a, b) {
            self.graph.update_edge(a, b, weight);
        } else {
            self.graph.add_edge(a, b, weight);
        }
    }

    // Example method to connect nodes based on a similarity threshold in a specific attribute.
    pub fn connect_on_threshold(&mut self, threshold: f32) {
        let indices: Vec<NodeIndex> = self.graph.node_indices().collect();
        for &i in &indices {
            for &j in &indices {
                if i != j {
                    let weight = self.calculate_similarity(i, j);
                    if weight > threshold {
                        self.add_or_update_edge(i, j, weight);
                    }
                }
            }
        }
    }

    // Calculates similarity between two participants based on some criteria.
    fn calculate_similarity(&self, a: NodeIndex, b: NodeIndex) -> f32 {
        let p1 = &self.graph[a];
        let p2 = &self.graph[b];
        1.0 / (1.0 + (p1.anxietyrec1().unwrap_or(0.0) - p2.anxietyrec1().unwrap_or(0.0)).abs())
    }
}

// In src/graph.rs
pub fn create_graph(participants: &[Participant]) -> MentalHealthGraph {
    let mut graph = MentalHealthGraph::new();
    for participant in participants {
        let _node = graph.add_participant(participant.clone());
    
    } 
    graph 
}