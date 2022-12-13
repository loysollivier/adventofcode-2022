fn is_visible(tree: u8, tree_line: &[&u8]) -> bool {
    if tree > **tree_line.iter().max().unwrap() {
        return true;
    }
    return false;
}

pub fn main() {
    const FSIZE: usize = 99;
    let mut forest: [[u8; FSIZE]; FSIZE] = [[0; FSIZE]; FSIZE];
    for (line_num, line) in include_str!("../input.txt").lines().enumerate() {
        forest[line_num] = line.chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>().try_into().unwrap();
    }
    // Remove mut
    let forest = forest;
    let mut visible = (FSIZE - 1) * 4;
    /*
    Right:  [1][1] VS forest[1][2..=99]
    left:   [1][1] VS forest[1][0..1]
    Top:    [1][1] VS forest[0..1][1]
    Bottom: [1][1] VS forest[2..=99][1]
    */
    for i in 1..FSIZE-1 {
        for j in 1..FSIZE-1 {
            let line = forest[i].iter().collect::<Vec<_>>();
            // Visible right
            if is_visible(forest[i][j], &line[j+1..FSIZE]) {
                visible += 1;
                // println!("Tree {} visible right - coordinates [{}][{}]", forest[i][j], i, j);
                continue;
            }
            // Visible left
            if is_visible(forest[i][j], &line[0..j]) {
                visible += 1;
                // println!("Tree {} visible left - coordinates [{}][{}]", forest[i][j], i, j);
                continue;
            }
            // Visible top
            let column = forest.iter().map(|t| t.iter().nth(j).unwrap()).collect::<Vec<_>>();
            if is_visible(forest[i][j], &column[0..i]) {
                visible += 1;
                // println!("Tree {} visible top - coordinates [{}][{}]", forest[i][j], i, j);
                continue;
            }
            // Visible bottom
            if is_visible(forest[i][j], &column[i+1..FSIZE]) {
                visible += 1;
                // println!("Tree {} visible bottom - coordinates [{}][{}]", forest[i][j], i, j);
                continue;
            }
        }
    }
    println!("Found {} trees visible.", visible);
}
