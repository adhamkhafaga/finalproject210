use petgraph::prelude::*;
use std::collections::HashMap;
use super::graph::MentalHealthGraph;
use crate::data::Participant;

// Calculate average scores for a given score selector
pub fn calculate_average_score(graph: &MentalHealthGraph, score_selector: fn(&Participant) -> Option<f32>) -> f32 {
    let sum: f32 = graph.graph.node_weights().filter_map(score_selector).sum();
    let count: f32 = graph.graph.node_weights().filter_map(score_selector).count() as f32;
    if count == 0.0 { 0.0 } else { sum / count }
}

// Function to find node with highest degree centrality
pub fn highest_degree_centrality(graph: &MentalHealthGraph) -> Option<NodeIndex> {
    graph.graph.node_indices().max_by_key(|&n| graph.graph.edges(n).count())
}

// Community detection using simple label propagation
pub fn detect_communities(graph: &MentalHealthGraph) -> Vec<usize> {
    let mut community_labels: Vec<usize> = vec![0; graph.graph.node_count()];
    let mut changed = true;
    while changed {
        changed = false;
        for node in graph.graph.node_indices() {
            let mut frequencies = std::collections::HashMap::new();
            for neighbor in graph.graph.neighbors(node) {
                *frequencies.entry(community_labels[neighbor.index()]).or_insert(0) += 1;
            }
            if let Some(&most_common) = frequencies.iter().max_by_key(|&(_, count)| count).map(|(label, _)| label) {
                if most_common != community_labels[node.index()] {
                    community_labels[node.index()] = most_common;
                    changed = true;
                }
            }
        }
    }
    community_labels
}

// Selector function for anxiety scores
fn anxiety_score(participant: &Participant) -> Option<f32> {
    participant.anxietyrec1()
}

// Analyzing the graph for average anxiety score
pub fn analyze_graph(graph: &MentalHealthGraph) {
    let average_anxiety = calculate_average_score(graph, anxiety_score);
    println!("Average anxiety score: {}", average_anxiety);

    if let Some(high_deg) = highest_degree_centrality(graph) {
        println!("Node with highest degree centrality has index: {:?}", high_deg);
    }

    let communities = detect_communities(graph);
    println!("Community detection (label propagation): {:?}", communities);
}

pub fn analyze_depression_by_age(participants: &[Participant]) -> HashMap<u32, f32> {
    let mut age_groups: HashMap<u32, Vec<f32>> = HashMap::new();
    for participant in participants {
        if let (Some(age), Some(depression)) = (participant.women_age(), participant.depression_rec1()) {
            age_groups.entry(age).or_insert_with(Vec::new).push(depression);
        }
    }

    let mut averages: HashMap<u32, f32> = HashMap::new();
    for (age, depressions) in age_groups {
        let average_depression: f32 = depressions.iter().sum::<f32>() / depressions.len() as f32;
        averages.insert(age, average_depression);
    }
    averages
}