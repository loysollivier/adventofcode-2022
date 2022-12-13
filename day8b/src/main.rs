fn is_visible(tree: u8, tree_line: &[&u8], reverse: bool) -> u32 {
    let mut view = 0;
    if reverse {
        let mut the_it = tree_line.iter().rev().peekable();
        while the_it.peek().is_some() {
            view += 1;
            if tree <= **the_it.next().unwrap() {
                break;
            }
        }
    } else {
        let mut the_it = tree_line.iter().peekable();
        while the_it.peek().is_some() {
            view += 1;
            if tree <= **the_it.next().unwrap() {
                break;
            }
        }
    }
    return view;
}

pub fn main() {
    const FSIZE: usize = 99;
    let mut forest: [[u8; FSIZE]; FSIZE] = [[0; FSIZE]; FSIZE];
    for (line_num, line) in include_str!("../input.txt").lines().enumerate() {
        forest[line_num] = line.chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>().try_into().unwrap();
    }
    // Remove mut
    let forest = forest;
    let mut v_dist = Vec::new();
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
            let right = is_visible(forest[i][j], &line[j+1..FSIZE], false);
            // Visible left
            let left = is_visible(forest[i][j], &line[0..j], true);
            // Visible top
            let column = forest.iter().map(|t| t.iter().nth(j).unwrap()).collect::<Vec<_>>();
            let top = is_visible(forest[i][j], &column[0..i], true);
            // Visible bottom
            let bottom = is_visible(forest[i][j], &column[i+1..FSIZE], false);
            v_dist.push(right * left * top * bottom);
        }
    }
    println!("Best scenic score: {}.", v_dist.iter().max().unwrap());
}
