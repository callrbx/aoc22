use std::collections::HashMap;

use crate::util;

const DAY_STR: &str = "Day 3";
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug)]
struct Rucksack {
    bag1: String,
    bag2: String,
    all: String,
}

#[derive(Debug)]
struct Group {
    ruck1: Rucksack,
    ruck2: Rucksack,
    ruck3: Rucksack,
}

impl Rucksack {
    fn new(items: &str) -> Self {
        if items.len() % 2 != 0 {
            eprintln!("Got odd item list!");
            std::process::exit(1);
        }
        let (i1, i2) = items.split_at(items.len() / 2);
        return Self {
            bag1: i1.to_string(),
            bag2: i2.to_string(),
            all: items.to_string(),
        };
    }

    fn item_priority(item: char) -> u32 {
        match ALPHABET.find(item) {
            Some(n) => n as u32 + 1,
            None => 0,
        }
    }

    fn priority(self) -> u32 {
        self.find_shared_items().iter().sum()
    }

    fn find_shared_items(self) -> Vec<u32> {
        let mut shared_items: HashMap<char, u32> = HashMap::new();

        for c in self.bag1.chars() {
            if self.bag2.contains(c) {
                shared_items.insert(c, Self::item_priority(c));
            }
        }

        return shared_items.values().cloned().collect();
    }
}

impl Group {
    fn new(r1: &str, r2: &str, r3: &str) -> Self {
        Self {
            ruck1: Rucksack::new(r1),
            ruck2: Rucksack::new(r2),
            ruck3: Rucksack::new(r3),
        }
    }

    fn priority(self) -> u32 {
        self.find_shared_items().iter().sum()
    }

    fn find_shared_items(self) -> Vec<u32> {
        let mut shared_items: HashMap<char, u32> = HashMap::new();

        for c in self.ruck1.all.chars() {
            if self.ruck2.all.contains(c) && self.ruck3.all.contains(c) {
                shared_items.insert(c, Rucksack::item_priority(c));
            }
        }
        return shared_items.values().cloned().collect();
    }
}

pub fn solve() -> String {
    let input_str = util::get_input("inputs/day3");
    let p1 = part1(&input_str);
    println!("\t{} Part 1: {:?}", DAY_STR, p1);
    let p2 = part2(&input_str);
    println!("\t{} Part 2: {:?}", DAY_STR, p2);

    return DAY_STR.to_string();
}

pub fn part1(input_str: &String) -> u32 {
    input_str
        .split("\n")
        .map(|line| Rucksack::new(line).priority())
        .sum()
}

pub fn part2(input_str: &String) -> u32 {
    input_str
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| match chunk.len() {
            3 => Group::new(chunk[0], chunk[1], chunk[2]).priority(),
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input_str = super::util::get_input("inputs/day3_test");

        let p1 = super::part1(&input_str);

        assert_eq!(p1, 157);
    }

    #[test]
    fn part2() {
        let input_str = super::util::get_input("inputs/day3_test");

        let p2 = super::part2(&input_str);

        assert_eq!(p2, 70);
    }
}
