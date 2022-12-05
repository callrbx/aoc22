use std::collections::VecDeque;

use crate::util;

const DAY_STR: &str = "Day 5";
const WIDTH: usize = 4;

#[derive(Clone)]
pub struct Yard {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<String>,
}

impl Yard {
    fn new(input_str: &String) -> Self {
        let (setup, moves) = match input_str.split_once("\n\n") {
            Some((s, m)) => (s, m),
            None => {
                eprintln!("Failed to parse init state");
                std::process::exit(1);
            }
        };

        let lines = setup.split("\n").collect::<Vec<&str>>();
        let n_stacks = (lines.last().unwrap().len() + 1) / WIDTH;

        let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(n_stacks);
        for _ in 0..n_stacks {
            stacks.push(VecDeque::new());
        }

        for line in &lines[0..lines.len() - 1] {
            let row = line.chars().collect::<Vec<char>>();
            let items = row.chunks(WIDTH);
            for (idx, c) in items.enumerate() {
                match c.len() {
                    3 | 4 => {
                        let i = c[1];
                        if i != ' ' {
                            stacks[idx].push_back(i);
                        }
                    }
                    _ => {
                        eprintln!("Found a weird sized chunk: {:?}", c);
                        std::process::exit(1);
                    }
                }
            }
        }

        return Self {
            stacks: stacks,
            moves: moves
                .split("\n")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        };
    }

    fn do_moves_part1(&mut self) {
        for mv in &self.moves {
            let parts = mv.split_whitespace().collect::<Vec<&str>>();
            if parts.len() == 0 {
                return;
            }
            match parts[0] {
                "move" => {
                    let n = parts[1].parse::<usize>().unwrap();
                    let from = parts[3].parse::<usize>().unwrap() - 1;
                    let to = parts[5].parse::<usize>().unwrap() - 1;

                    for _ in 0..n {
                        let t = self.stacks[from].pop_front().unwrap();
                        self.stacks[to].push_front(t);
                    }
                }
                _ => {
                    eprintln!("Unsupported: {}", parts[0])
                }
            }
        }
    }

    fn do_moves_part2(&mut self) {
        for mv in &self.moves {
            let parts = mv.split_whitespace().collect::<Vec<&str>>();
            if parts.len() == 0 {
                return;
            }
            match parts[0] {
                "move" => {
                    let n = parts[1].parse::<usize>().unwrap();
                    let from = parts[3].parse::<usize>().unwrap() - 1;
                    let to = parts[5].parse::<usize>().unwrap() - 1;

                    let mut x: Vec<char> = Vec::new();

                    for _ in 0..n {
                        x.push(self.stacks[from].pop_front().unwrap());
                    }

                    x.reverse();

                    for c in x {
                        self.stacks[to].push_front(c);
                    }
                }
                _ => {
                    eprintln!("Unsupported: {}", parts[0])
                }
            }
        }
    }

    fn get_tops(&self) -> String {
        return self
            .stacks
            .iter()
            .map(|s| s.front().unwrap())
            .collect::<String>();
    }
}

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/day5");

    let base_yard = Yard::new(&input_str);

    let p1 = part1(base_yard.clone());
    let p2 = part2(base_yard.clone());

    return (
        DAY_STR.to_string(),
        String::from(format!("\n\tPart 1: {}\n\tPart 2: {}", p1, p2)),
    );
}

pub fn part1(mut yard: Yard) -> String {
    yard.do_moves_part1();
    return yard.get_tops();
}

pub fn part2(mut yard: Yard) -> String {
    yard.do_moves_part2();
    return yard.get_tops();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn combined_tests() {
        let input_str = util::get_input("inputs/day5_tests");

        let base_yard = Yard::new(&input_str);

        let p1 = part1(base_yard.clone());
        let p2 = part2(base_yard.clone());

        assert_eq!(p1, "CMZ");
        assert_eq!(p2, "MCD");
    }
}
