use crate::util;

const DAY_STR: &str = "Day 4";

#[derive(PartialEq, Debug, Clone, Copy)]
struct Assignment {
    low: u32,
    high: u32,
}

impl Assignment {
    fn new(assign_str: &str) -> Self {
        let (low, high) = assign_str.split_once("-").unwrap();
        Self {
            low: low.parse::<u32>().unwrap(),
            high: high.parse::<u32>().unwrap(),
        }
    }

    fn fully_contains(&self, other: &Self) -> bool {
        return self.low <= other.low && self.high >= other.high;
    }

    fn overlaps(&self, other: &Self) -> bool {
        return self.low <= other.high && self.high >= other.low;
    }
}

struct Schedule {
    assignments: Vec<Assignment>,
    ol_pairs: u32,
    ol_sections: u32,
}
impl Schedule {
    fn new() -> Self {
        return Self {
            assignments: Vec::new(),
            ol_pairs: 0,
            ol_sections: 0,
        };
    }

    fn add(&mut self, line: &str) {
        match line.split_once(",") {
            Some((a1, a2)) => {
                let a1r = Assignment::new(a1);
                let a2r = Assignment::new(a2);
                self.assignments.push(a1r);
                self.assignments.push(a2r);
                if a1r.fully_contains(&a2r) || a2r.fully_contains(&a1r) {
                    self.ol_pairs += 1
                }
                if a1r.overlaps(&a2r) || a2r.overlaps(&a1r) {
                    self.ol_sections += 1
                }
            }
            None => {}
        }
    }
}

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/day4");
    let mut sched = Schedule::new();

    for line in input_str.split("\n") {
        sched.add(line);
    }

    return (
        DAY_STR.to_string(),
        String::from(format!(
            "\n\tPart 1: {:?}\n\tPart 2: {:?}",
            sched.ol_pairs, sched.ol_sections
        )),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input_str = super::util::get_input("inputs/day4_test");

        let mut sched = super::Schedule::new();

        for line in input_str.split("\n") {
            sched.add(line);
        }

        assert_eq!(sched.ol_pairs, 2);
        assert_eq!(sched.ol_sections, 4)
    }
}
