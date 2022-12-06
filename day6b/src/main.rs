pub fn main() {
    let full_code: Vec<char> = include_str!("../input.txt").lines().next().unwrap().chars().collect();
    let size = 14;
    'code_loop: for e_seq in full_code.windows(size).enumerate() {
        let seq = e_seq.1;
        'match_loop: for i in 0..size {
            for j in i+1..size {
                if seq[i] == seq[j] {
                    break 'match_loop;
                }
            }
            if i == size-1 {
                println!("Position: {} sequence: {:?}", e_seq.0 + size, seq);
                break 'code_loop;
            }
        }
    }
}
