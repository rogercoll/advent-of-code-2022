use std::collections::HashMap;

fn get_dirs_map(input: String) -> HashMap<String, usize> {
    let mut dirs = Vec::<String>::new();
    let mut dirs_size = HashMap::<String, usize>::new();

    input.split('\n').for_each(|line| {
        let cmd_output = line
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        match (cmd_output[0].as_str(), cmd_output[1].as_str()) {
            ("$", "cd") => {
                if cmd_output[2] == ".." {
                    let exit_dir_size = *dirs_size.get(&dirs.join("/")).unwrap();
                    dirs.pop();
                    dirs_size
                        .entry(dirs.join("/"))
                        .and_modify(|size| *size += exit_dir_size)
                        .or_insert(exit_dir_size);
                } else {
                    dirs.push(cmd_output[2].clone());
                };
            }
            ("$", "ls") => (),
            ("dir", _) => (),
            _ => {
                let file_size = cmd_output[0].parse::<usize>().unwrap();
                dirs_size
                    .entry(dirs.join("/"))
                    .and_modify(|size| *size += file_size)
                    .or_insert(file_size);
            }
        }
    });

    // update upper dir size from last scan
    while !dirs.is_empty() {
        let exit_dir_size = *dirs_size.get(&dirs.join("/")).unwrap();
        dirs.pop();
        dirs_size
            .entry(dirs.join("/"))
            .and_modify(|size| *size += exit_dir_size)
            .or_insert(exit_dir_size);
    }

    dirs_size
}

pub fn part1(input: String) -> String {
    get_dirs_map(input)
        .iter()
        .map(|(_, value)| value)
        .filter(|value| **value <= 100000)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let dirs = get_dirs_map(input);
    let unused_space = 70000000_usize - dirs.get("/").unwrap();
    dirs.iter()
        .map(|(_, value)| value)
        .filter(|value| (unused_space + **value) >= 30000000_usize)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "95437",
            part1("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k".to_string())
        );
        assert_eq!(
            "24933642",
            part2("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k".to_string())
        );
    }
}
