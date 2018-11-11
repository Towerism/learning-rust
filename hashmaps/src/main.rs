use std::collections::HashMap;

fn main() {
    let team1 = String::from("Blue");
    let team2 = String::from("Yellow");

    let mut s1 = HashMap::new();

    // team1/team2 must be borrowed, otherwise
    // they will be moved and references held
    // by team1/team2 invalidated.
    s1.insert(&team1, 10);

    s1.entry(&team1).or_insert(50);
    s1.entry(&team2).or_insert(50);

    println!("s1:");
    for (key, value) in &s1 {
        println!("{}: {}", key, value);
    }

    // still have to move team1/team2,
    // because they are still being borrowed by s1
    let teams = vec![&team1, &team2];
    let initial_scores = vec![10, 50];
    let s2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("s2: {:?}", s2);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("word count:");
    println!("{:?}", map);
}
