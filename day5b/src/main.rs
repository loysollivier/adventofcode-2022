use regex::Regex;

pub fn main() {
    // Hardcode the stacks, faster than writing to code to parse them
    let stack1 = vec!['V', 'C', 'D', 'R', 'Z', 'G', 'B', 'W'];
    let stack2 = vec!['G', 'W', 'F', 'C', 'B', 'S', 'T', 'V'];
    let stack3 = vec!['C', 'B', 'S', 'N', 'W'];
    let stack4 = vec!['Q', 'G', 'M', 'N', 'J', 'V', 'C', 'P'];
    let stack5 = vec!['T', 'S', 'L', 'F', 'D', 'H', 'B'];
    let stack6 = vec!['J', 'V', 'T', 'W', 'M', 'N'];
    let stack7 = vec!['P', 'F', 'L', 'C', 'S', 'T', 'G'];
    let stack8 = vec!['B', 'D', 'Z'];
    let stack9 = vec!['M', 'N', 'Z', 'W'];
    let mut platforms = [stack1, stack2, stack3, stack4,
                         stack5, stack6, stack7, stack8, stack9];

    let re = Regex::new(r"move ([0-9]*) from ([0-9]*) to ([0-9]*)").unwrap();
    for line in include_str!("../input.txt").lines() {
        let orders = re.captures(line).unwrap();
        let count = orders.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let origin: usize = orders.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let destination: usize = orders.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        let new_length = platforms[origin].len().saturating_sub(count);
        let mut temp = platforms[origin].split_off(new_length);
        platforms[destination].append(&mut temp);
    }
    print!("Top of stacks: ");
    for stack in platforms {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
