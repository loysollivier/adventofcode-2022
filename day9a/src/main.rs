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

pub fn main() {
    let mut all_tail_pos = HashSet::from([(0,0)]);
    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);
    let re_move = Regex::new(r"^([A-Z]) ([0-9]*)$").unwrap();
    for line in include_str!("../input.txt").lines() {
        let re_capture = re_move.captures(line).unwrap();
        let direction = re_capture.get(1).unwrap().as_str();
        let distance = re_capture.get(2).unwrap().as_str().parse::<u8>().unwrap();
        // println!("Move {} {} times", direction, distance);
        for _ in 0..distance {
            match direction {
                "R" => head_pos.0 += 1,
                "L" => head_pos.0 -= 1,
                "U" => head_pos.1 += 1,
                "D" => head_pos.1 -= 1,
                _ => panic!()
            }
            if !touches(&head_pos, &tail_pos) {
                match direction {
                    "R" => tail_pos = (tail_pos.0+1, head_pos.1),
                    "L" => tail_pos = (tail_pos.0-1, head_pos.1),
                    "U" => tail_pos = (head_pos.0, tail_pos.1+1),
                    "D" => tail_pos = (head_pos.0, tail_pos.1-1),
                    _ => panic!()
                }
                all_tail_pos.insert(tail_pos);
            }
        }
    }
    println!("{}", all_tail_pos.len());
}

/*

##R##

T (3,0) -> (4,1)
H (5,1) -> (5,1)

......
......
......
.....H
s..T..

##L##

T (4,3) -> (3,4)
H (2,4) -> (2,4)

..H...
....T.
......
......
s.....

##U##

T (1,1) -> (2,2)
H (2,3) -> (2,3)

......
..H...
......
.T....
s.....

##D##

T (2,3) -> (1,2)
H (1,1) -> (1,1)

......
..T...
......
.H....
s.....

*/
