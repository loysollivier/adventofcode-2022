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
            // Up one level
            "$ cd .." => {
                level -= 1;
                dir_path.pop();
            },
            // Down one level
            line if re_cd_to.is_match(line) => {
                let directory = re_cd_to.captures(line).unwrap().get(1).unwrap().as_str();
                level += 1;
                if filesystem.len() <= level {
                    filesystem.push(Vec::from([]));
                }
                filesystem[level].push((String::from(directory), 0));
                dir_path.push(filesystem[level].len() - 1);
            },
            // Add files
            "$ ls" => (),
            line if re_file_size.is_match(line) => {
                let fsize: u32 = re_file_size.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
                for a_level in 0..=level {
                    let fs_iter = &mut filesystem[a_level][dir_path[a_level]];
                    fs_iter.1 += fsize;
                }
            }
            // Skip rest
            _ => ()
        }
    }
    let disk_size = 70000000;
    let unused_required = 30000000;
    let used_space = filesystem[0][0].1;
    let unused = disk_size - used_space;
    let mut smallest_dir = used_space;

    for level in filesystem {
        for (_, size) in level {
            if unused + size > unused_required {
                if size < smallest_dir {
                    smallest_dir = size;
                }
            }
        }
    }
    println!("Smallest dir to remove: {}", smallest_dir);
}
