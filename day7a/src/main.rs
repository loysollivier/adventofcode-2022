use regex::Regex;

/*
Commands:
 - "$ cd ..": move one directory up
 - "$ cd [a-z]*": move one directory down
 - "$ ls": list files/directories
*/

pub fn main() {
    let re_cd_to = Regex::new(r"^\$ cd ([a-z]*)$").unwrap();
    let re_file_size = Regex::new(r"^([0-9]*) .*$").unwrap();

    let mut filesystem: Vec<Vec<(String, u32)>>= Vec::new();
    filesystem.push(Vec::from([(String::from("/"), 0u32)]));
    let mut level: usize = 0;
    let mut dir_path: Vec<usize> = Vec::from([0]);

    for line in include_str!("../input.txt").lines() {
        match line {
            "$ cd .." => {
                level -= 1;
                dir_path.pop();
            },
            line if re_cd_to.is_match(line) => {
                let directory = re_cd_to.captures(line).unwrap().get(1).unwrap().as_str();
                level += 1;
                // Create new level
                if filesystem.len() <= level {
                    filesystem.push(Vec::from([]));
                }
                // Add directory to Vec
                // WTF the system can have several folders with same name on same level...
                // My initial solution was adding sizes for duplicate folder names...
                // let find_index = filesystem[level].iter().position(|x| x.0 == directory);
                // if find_index.is_none() {
                filesystem[level].push((String::from(directory), 0));
                dir_path.push(filesystem[level].len() - 1);
                // } else {
                //     dir_path.push(find_index.unwrap());
                // }
            },
            "$ ls" => (),
            line if re_file_size.is_match(line) => {
                let fsize: u32 = re_file_size.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
                for a_level in 0..=level {
                    let fs_iter = &mut filesystem[a_level][dir_path[a_level]];
                    fs_iter.1 += fsize;
                }
            }
            _ => ()
        }
    }
    let mut total_size = 0;
    for level in filesystem {
        for (_, size) in level {
            if size <= 100000 {
                total_size += size;
            }
        }
    }
    println!("Total size: {total_size}")
}
