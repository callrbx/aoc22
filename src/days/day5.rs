use regex::Regex;
use std::collections::VecDeque;

use crate::util;

const DAY_STR: &str = "Day 5";
const WIDTH: usize = 4;

#[derive(Clone, Debug)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
pub struct Yard {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
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
            for n in 0..n_stacks {
                let pos_char = n * 4 + 1;
                if row[pos_char] != ' ' {
                    stacks[n].push_front(row[pos_char]);
                }
            }
        }

        let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let mut vec_moves = Vec::new();

        for mv in moves.split("\n") {
            if mv.len() < 2 {
                break;
            }
            let cap = r.captures(mv).unwrap();
            vec_moves.push(Move {
                count: cap[1].parse().unwrap(),
                from: cap[2].parse::<usize>().unwrap() - 1,
                to: cap[3].parse::<usize>().unwrap() - 1,
            });
        }

        return Self {
            stacks: stacks,
            moves: vec_moves,
        };
    }

    fn do_moves(&mut self, reverse: bool) {
        for mv in &self.moves {
            let from = self.stacks.get_mut(mv.from).unwrap();
            let mut to_move = from.split_off(from.len() - mv.count);

            if reverse {
                to_move.make_contiguous().reverse();
            }

            self.stacks[mv.to].append(&mut to_move);
        }
    }

    fn get_tops(&self) -> String {
        return self
            .stacks
            .iter()
            .map(|s| s.back().unwrap())
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
    yard.do_moves(true);
    return yard.get_tops();
}

pub fn part2(mut yard: Yard) -> String {
    yard.do_moves(false);
    return yard.get_tops();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parts1_2() {
        let input_str = util::get_input("inputs/day5_test");

        let base_yard = Yard::new(&input_str);

        let p1 = part1(base_yard.clone());
        let p2 = part2(base_yard.clone());

        assert_eq!(p1, "CMZ");
        assert_eq!(p2, "MCD");
    }
}
