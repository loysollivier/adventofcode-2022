pub fn main() {
    let full_code: Vec<char> = include_str!("../input.txt").lines().next().unwrap().chars().collect();
    for e_seq in full_code.windows(4).enumerate() {
        let seq = e_seq.1;
        if seq[0] != seq[1] && seq[0] != seq[2] && seq[0] != seq[3] && seq[1] != seq[2] && seq[1] != seq[3] && seq[2] != seq[3] {
            println!("Position: {} sequence: {:?}", e_seq.0 + 4, seq);
            break;
        }
    }
}
