use std::collections::HashSet;

use crate::util;

const DAY_STR: &str = "Day 9";

#[derive(Debug)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
struct Move {
    dir: Dir,
    dist: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sim {
    knots: Vec<Knot>,
    knot_hist_tail: HashSet<Knot>,
    knot_hist: HashSet<Knot>,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    // returns if it moved
    fn follow(&mut self, head: Knot) -> bool {
        let (dx, dy) = (head.x - self.x, head.y - self.y);
        if dx.abs() > 1 || dy.abs() > 1 {
            self.x += dx.signum();
            self.y += dy.signum();
            return true;
        } else {
            return false;
        }
    }
}

impl Dir {
    fn from(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => {
                eprintln!("Unknown Dir: {}", s);
                std::process::exit(1);
            }
        }
    }
}

impl Move {
    fn new(line: &str) -> Option<Self> {
        match line.split_once(" ") {
            Some((dir, dist)) => Some(Self {
                dir: Dir::from(dir),
                dist: dist.parse::<i32>().unwrap(),
            }),
            None => None,
        }
    }
}

impl Sim {
    fn new() -> Self {
        let mut new_self = Self {
            knots: vec![Knot::new(0, 0); 10],
            knot_hist_tail: HashSet::new(),
            knot_hist: HashSet::new(),
        };

        new_self
            .knot_hist_tail
            .insert(*new_self.knots.first().unwrap());
        new_self.knot_hist.insert(*new_self.knots.first().unwrap());
        return new_self;
    }
    fn step(&mut self, mv: Move) {
        // execute steps
        for _ in 0..mv.dist {
            let head = self.knots.first_mut().unwrap();

            // move head knot
            match mv.dir {
                Dir::Up => head.y += 1,
                Dir::Down => head.y -= 1,
                Dir::Left => head.x -= 1,
                Dir::Right => head.x += 1,
            };

            // move follow on knots
            for k in 1..self.knots.len() {
                let prev = self.knots[k - 1];
                if self.knots[k].follow(prev) && k == 1 {
                    // keep track of primary tail - part 1
                    self.knot_hist_tail.insert(self.knots[k]);
                }
            }
            // second part
            self.knot_hist.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn do_move(&mut self, move_str: &str) -> bool {
        match Move::new(move_str) {
            Some(mv) => {
                self.step(mv);
                return true;
            }
            None => return false,
        }
    }
}

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/day9");
    let (p1, p2) = parts1_2(&input_str);

    return (
        DAY_STR.to_string(),
        String::from(format!("\n\tPart 1: {:?}\n\tPart 2: {:?}", p1, p2)),
    );
}

pub fn parts1_2(input_str: &String) -> (usize, usize) {
    let mut sim = Sim::new();

    for move_str in input_str.split("\n") {
        sim.do_move(move_str);
    }

    return (sim.knot_hist_tail.len(), sim.knot_hist.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn parts1_2() {
        let input_str = super::util::get_input("inputs/day9_test");

        let (p1, p2) = super::parts1_2(&input_str);

        assert_eq!(p1, 88);
        assert_eq!(p2, 36);
    }
}
