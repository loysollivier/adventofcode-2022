use regex::Regex;

pub fn main() {
    const CYCLES: [i16;6] = [20, 60, 100, 140, 180, 220];
    let mut signal_strength: Vec::<(i16, i16)> = Vec::new();
    let mut x_val = 1;
    let mut curr_cycle = 0;
    let re_instr = Regex::new(r"^([a-z]*) *(-*[0-9]*)?$").unwrap();
    for line in include_str!("../input.txt").lines() {
        let re_capture = re_instr.captures(line).unwrap();
        let instr = re_capture.get(1).unwrap().as_str();
        match instr {
            "addx" => {
                let operand = re_capture.get(2).unwrap().as_str().parse::<i8>().unwrap();
                for _ in 0..2 {
                    curr_cycle += 1;
                    if CYCLES.contains(&curr_cycle) {
                        signal_strength.push((curr_cycle, x_val));
                    }
                    // println!("{curr_cycle}: {x_val}");
                }
                x_val += operand as i16;
            },
            "noop" => {
                curr_cycle += 1;
                if CYCLES.contains(&curr_cycle) {
                    signal_strength.push((curr_cycle, x_val));
                }
                // println!("{curr_cycle}: {x_val}");
            },
            _ => panic!()
        }
    }
    // println!("{:?}", signal_strength);
    println!("{}", signal_strength.into_iter().map(|(a, b)| a * b).sum::<i16>());
}
