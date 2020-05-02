use std::collections::HashMap;

pub fn hashMapExamples() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Madrid"), 1);
    scores.insert(String::from("Galatasaray"), 0);

    println!("{:?}", scores);

    let teams = vec![String::from("blue"), String::from("Yellow")];

    let initial_scores = vec![10, 50];

    let scores_teams_tuple = teams.iter().zip(initial_scores.iter());

    println!("{:?}", scores_teams_tuple);

    let scores_teams: HashMap<_, _> = scores_teams_tuple.collect();

    println!("{:?}", scores_teams);

    let score_1 = scores_teams.get(&String::from("blue"));
    println!("score_1: {:?}", score_1);
}
