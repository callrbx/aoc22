use itertools::Itertools;

use crate::util;

const DAY_STR: &str = "Day 7";
const TOTAL_SIZE: usize = 70_000_000;
const NEED_SIZE: usize = 30_000_000;

#[derive(Debug)]
struct Dir {
    files: Vec<usize>,
    dirs: Vec<Dir>,
}

impl Dir {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn add_dir(&mut self, dir: Dir) {
        self.dirs.push(dir);
    }

    fn add_file(&mut self, file: usize) {
        self.files.push(file);
    }

    fn dir_size(&self) -> usize {
        let file_size = self.files.iter().sum::<usize>();
        let dir_size = self.dirs.iter().map(|dir| dir.dir_size()).sum::<usize>();
        file_size + dir_size
    }

    fn recurse_size(&self) -> Vec<usize> {
        self.dirs
            .iter()
            .flat_map(|dir| dir.recurse_size())
            .chain([self.dir_size()])
            .collect::<Vec<usize>>()
    }
}

fn cmd_parser(lines: String) -> Dir {
    let mut cwd = Vec::new();
    cwd.push(Dir::new());

    // create nested dir structure
    for line in lines.split("\n") {
        if line.len() < 2 {
            break;
        }
        match line {
            "$ ls" | "$ cd /" => {}
            "$ cd .." => {
                let last_dir = cwd.pop().unwrap();
                cwd.last_mut().unwrap().add_dir(last_dir);
            }
            line if line.starts_with("dir ") => {}
            line if line.starts_with("$ cd") => {
                cwd.push(Dir::new());
            }
            line => {
                let file = line
                    .split_once(' ')
                    .map(|(size, _)| {
                        size.parse()
                            .expect(format!("Size is wrong {}", size).as_str())
                    })
                    .expect("Parse error");
                cwd.last_mut().unwrap().add_file(file);
            }
        }
    }

    while cwd.len() > 1 {
        let directory = cwd.pop().unwrap();
        cwd.last_mut().unwrap().add_dir(directory);
    }

    cwd.into_iter().next().unwrap()
}

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/day7");

    let (p1, p2) = combined_solve(input_str);

    return (
        DAY_STR.to_string(),
        String::from(format!("\n\tPart 1: {:?}\n\tPart 2: {:?}", p1, p2)),
    );
}

fn combined_solve(input_str: String) -> (usize, usize) {
    let root = cmd_parser(input_str.to_string());

    let p1: usize = root
        .recurse_size()
        .iter()
        .filter(|&&size| size <= 100_000)
        .sum();

    let need_to_free = NEED_SIZE - (TOTAL_SIZE - root.dir_size());

    let dir_size = *root
        .recurse_size()
        .iter()
        .filter(|&&size| size >= need_to_free)
        .sorted()
        .next()
        .unwrap();

    let p2 = dir_size;

    return (p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parts1_2() {
        let input_str = util::get_input("inputs/day7_test");

        let (p1, p2) = combined_solve(input_str);

        assert_eq!(p1, 95437);
        assert_eq!(p2, 24933642);
    }
}
