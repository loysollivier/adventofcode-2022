use std::collections::HashMap;
use std::iter::zip;

pub fn main() {
    let mut tot = 0;
    let mut score: HashMap<char,u16> = HashMap::new();
    for (s, l) in zip(1..=26,'a'..='z') {
        score.insert(l,s);
    }
    for (s, l) in zip(27..=52,'A'..='Z') {
        score.insert(l,s);
    }
    for line in include_str!("../input.txt").lines() {
        let size = line.len() / 2;
        let slice1: Vec<char> = line[ .. size].chars().collect();
        let slice2: Vec<char> = line[size .. ].chars().collect();

        'out_loop: for item_a in slice1 {
            for item_b in &slice2 {
                if item_a == *item_b {
                    tot += *(score.get(&item_a).unwrap_or(&0u16));
                    break 'out_loop;
                }
            }
        }
    }
    println!("Total: {}", tot)
}
