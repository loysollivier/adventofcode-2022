use std::collections::HashMap;
use std::iter::zip;

fn main() {
    let mut tot = 0;
    let mut score: HashMap<char,u16> = HashMap::new();
    for (s, l) in zip(1..=26,'a'..='z') {
        score.insert(l,s);
    }
    for (s, l) in zip(27..=52,'A'..='Z') {
        score.insert(l,s);
    }
    let mut file_it = include_str!("../input.txt").lines().peekable();
    while file_it.peek().is_some() {
        let elf1: Vec<char> = file_it.next().unwrap().chars().collect();
        let elf2: Vec<char> = file_it.next().unwrap().chars().collect();
        let elf3: Vec<char> = file_it.next().unwrap().chars().collect();
        for item in elf1 {
            if elf2.contains(&item) && elf3.contains(&item) {
                tot += *(score.get(&item).unwrap_or(&0u16));
                break;
            }
        }
    }
    println!("Total: {}", tot)
}
