fn main() {
    let mut max = 0;
    let mut count = 0;
    for line in include_str!("../input.txt").lines() {
        let x = line.parse::<u32>();
        match x {
            Ok(x) => {
                count += x;
                if count > max { max = count };
            },
            Err(_) => count = 0,
        }
    }
    println!("{}", max);
}
