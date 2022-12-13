use regex::Regex;

fn in_sprite(cycle: i16, sprite: i16) -> bool {
    if cycle >= sprite - 1 && cycle <= sprite + 1 {
        return true;
    }
    return false;
}

pub fn main() {
    const CRT: (usize, usize) = (40, 6);
    let mut drawing: Vec::<char> = Vec::new();
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
                    if in_sprite(curr_cycle, x_val) {
                        drawing.push('#');
                    } else {
                        drawing.push('.');
                    }
                    curr_cycle = (curr_cycle + 1) % CRT.0 as i16;
                }
                x_val += operand as i16;
            },
            "noop" => {
                if in_sprite(curr_cycle, x_val) {
                    drawing.push('#');
                } else {
                    drawing.push('.');
                }
                curr_cycle = (curr_cycle + 1) % CRT.0 as i16;
            },
            _ => panic!()
        }
    }
    for i in 0..CRT.1 {
        for j in 0..CRT.0 {
            print!("{}", drawing[i*CRT.0+j]);
        }
        println!();
    }
}
