use std::collections::HashMap;

// Rock     A X
// Paper    B Y
// Scissors C Z

pub fn main() {
    let scoring_table = HashMap::from([
        ("A X", 1+3), ("A Y", 2+6), ("A Z", 3+0),
        ("B X", 1+0), ("B Y", 2+3), ("B Z", 3+6),
        ("C X", 1+6), ("C Y", 2+0), ("C Z", 3+3),
    ]);
    let mut tot_score = 0;
    for line in include_str!("../input.txt").lines() {
        tot_score += scoring_table.get(line).unwrap();
    }
    println!("Score: {}", tot_score);
}
