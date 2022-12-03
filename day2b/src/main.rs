use std::collections::HashMap;

// Rock     A
// Paper    B
// Scissors C
// Lose     X
// Draw     Y
// Win      Z

fn main() {
    let scoring_table = HashMap::from([
        ("A X", 3+0), ("A Y", 1+3), ("A Z", 2+6),
        ("B X", 1+0), ("B Y", 2+3), ("B Z", 3+6),
        ("C X", 2+0), ("C Y", 3+3), ("C Z", 1+6),
    ]);
    let mut tot_score = 0;
    for line in include_str!("../input.txt").lines() {
        match scoring_table.get(line) {
            Some(score) => {
                // println!("{line}: {score}");
                tot_score += score;
            },
            None => panic!()
        }
    }
    println!("Score: {}", tot_score);
}
