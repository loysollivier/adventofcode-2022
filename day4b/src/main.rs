fn main() {
    let mut tot = 0;
    for line in include_str!("../input.txt").lines() {
        let v: Vec<u8> = line.split(&[',', '-'][..]).map(|x| x.parse::<u8>().unwrap()).collect();
        let (low1, high1, low2, high2) = (v[0], v[1], v[2], v[3]);
        if low2 <= low1 && low1 <= high2 || low2 <= high1 && high1 <= high2 {
            tot += 1;
        } else if low1 <= low2 && low2 <= high1 || low1 <= high2 && high2 <= high1 {
            tot += 1;
        }
    }
    println!("Total: {}", tot);
}
