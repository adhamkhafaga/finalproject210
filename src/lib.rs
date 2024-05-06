pub mod data;
pub mod graph;
pub mod analysis;

pub use data::Participant;
pub use graph::MentalHealthGraph;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::data::Participant;
    use crate::analysis;

    #[test]
    fn test_analyze_depression_by_age() {
        let participants = vec![
            Participant {
                f_id: "1".to_string(),
                part1_country: "Country1".to_string(),
                women_age: Some(30),
                depression_rec1: Some(2.0),
                ..Default::default()
            },
            Participant {
                f_id: "2".to_string(),
                part1_country: "Country2".to_string(),
                women_age: Some(30),
                depression_rec1: Some(4.0),
                ..Default::default()
            },
            Participant {
                f_id: "3".to_string(),
                part1_country: "Country3".to_string(),
                women_age: Some(40),
                depression_rec1: Some(5.0),
                ..Default::default()
            },
        ];

        let age_groups = analysis::analyze_depression_by_age(&participants);

        let mut expected_results = HashMap::new();
        expected_results.insert(30, 3.0); // Average of 2.0 and 4.0
        expected_results.insert(40, 5.0); // Only one entry

        for (age, &expected_average) in &expected_results {
            assert_eq!(*age_groups.get(age).unwrap(), expected_average, "Average depression score for age {} does not match expected.", age);
        }
    }
}
