use regex::Regex;
use std::collections::HashSet;

fn touches(head: &(i16, i16), tail: &(i16, i16)) -> bool {
    let x: i16 = head.0 - tail.0;
    let y: i16 = head.1 - tail.1;
    if x.abs() > 1 || y.abs() > 1 {
        return false;
    }
    return true;
}

fn inc_step(prev: i16, curr: i16) -> i16 {
    if prev - curr > 0 {
        return 1
    }
    return -1
}

pub fn main() {
    const ROPESIZE: usize = 10;
    let mut all_tail_pos = HashSet::from([(0,0)]);
    let mut knot_pos: [(i16, i16);ROPESIZE] = [(0,0);ROPESIZE];
    let re_move = Regex::new(r"^([A-Z]) ([0-9]*)$").unwrap();
    for line in include_str!("../input.txt").lines() {
        let re_capture = re_move.captures(line).unwrap();
        let direction = re_capture.get(1).unwrap().as_str();
        let distance = re_capture.get(2).unwrap().as_str().parse::<u8>().unwrap();
        // println!("Move {} {} times", direction, distance);
        for _ in 0..distance {
            match direction {
                "R" => knot_pos[0].0 += 1,
                "L" => knot_pos[0].0 -= 1,
                "U" => knot_pos[0].1 += 1,
                "D" => knot_pos[0].1 -= 1,
                _ => panic!()
            }
            for i in 1..ROPESIZE {
                let prev = knot_pos[i - 1];
                let mut curr = knot_pos[i];
                if !touches(&prev, &curr) {
                    if prev.0 != curr.0 && prev.1 != curr.1 {
                        // Diagonal move, move both (x,y)
                        curr.0 += inc_step(prev.0, curr.0);
                        curr.1 += inc_step(prev.1, curr.1);
                    } else if prev.0 == curr.0 {
                        // Move Y
                        curr.1 += inc_step(prev.1, curr.1);
                    } else {
                        // Move X
                        curr.0 += inc_step(prev.0, curr.0);
                    }
                    knot_pos[i] = curr;
                    if i == ROPESIZE-1 {
                        all_tail_pos.insert(knot_pos[i]);
                    }
                }
            }
        }
    }
    // println!("{:?}", all_tail_pos);
    println!("{:?}", all_tail_pos.len());
}
