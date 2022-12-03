use crate::util;

const DAY_STR: &str = "Day 3";
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug)]
struct Rucksack {
    bag1: Vec<u8>,
    bag2: Vec<u8>,
    all: Vec<u8>,
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
            bag1: Vec::from(i1),
            bag2: Vec::from(i2),
            all: Vec::from(items),
        };
    }

    fn priority(self) -> u32 {
        self.find_shared_items()
            .iter()
            .map(|c| Rucksack::item_priority(*c))
            .sum()
    }

    fn find_shared_items(self) -> Vec<u8> {
        let mut shared_items: Vec<u8> = Vec::new();

        for c in self.bag1 {
            if self.bag2.contains(&c) && !shared_items.contains(&c) {
                shared_items.push(c);
            }
        }

        return shared_items;
    }

    fn item_priority(item: u8) -> u32 {
        match ALPHABET.find(item as char) {
            Some(n) => n as u32 + 1,
            None => 0,
        }
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
        self.find_shared_items()
            .iter()
            .map(|c| Rucksack::item_priority(*c))
            .sum()
    }

    fn find_shared_items(self) -> Vec<u8> {
        let mut shared_items: Vec<u8> = Vec::new();

        for c in self.ruck1.all {
            if self.ruck2.all.contains(&c)
                && self.ruck3.all.contains(&c)
                && !shared_items.contains(&c)
            {
                shared_items.push(c);
            }
        }
        return shared_items;
    }
}

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/day3");
    let p1 = part1(&input_str);
    let p2 = part2(&input_str);

    return (
        DAY_STR.to_string(),
        String::from(format!("\n\tPart 1: {:?}\n\tPart 2: {:?}", p1, p2)),
    );
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
