fn main() {
    let mut max: [u32; 3] = [0; 3];
    let mut count = 0;
    for line in include_str!("../input.txt").lines() {
        let x = line.parse::<u32>();
        match x {
            Ok(x) => count += x,
            Err(_) => {
                println!("{}", count);
                match count {
                    count if count > max[2] => {max[0] = max[1]; max[1] = max[2]; max[2] = count},
                    count if count > max[1] => {max[0] = max[1]; max[1] = count},
                    count if count > max[0] => max[0] = count,
                    _ => {},
                };
                count = 0;
            },
        }
    }
    println!("{:?}", max);
    let tot: u32 = max.iter().sum();
    println!("{}", tot);
}
