use crate::util;

const WON: u32 = 6;
const LOST: u32 = 0;
const TIE: u32 = 3;

#[derive(PartialEq)]
enum Play {
    ROCK = 1,
    PAPER = 2,
    SCISCORS = 3,
}

impl Play {
    fn value(self) -> u32 {
        match self {
            Play::ROCK => 1,
            Play::PAPER => 2,
            Play::SCISCORS => 3,
        }
    }
}

struct Action {
    name: Play,
    counter: Play,
    score: u32,
}

struct Round {
    p1: Action,
    p2: Action,
}

impl Action {
    fn new(action: &str) -> Self {
        let action = match action {
            "A" | "X" => Play::ROCK,
            "B" | "Y" => Play::PAPER,
            "C" | "Z" => Play::SCISCORS,
            _ => std::process::exit(1),
        };

        let counter = match action {
            Play::ROCK => Play::PAPER,
            Play::PAPER => Play::SCISCORS,
            Play::SCISCORS => Play::ROCK,
        };

        let score = match action {
            Play::ROCK => 1,
            Play::PAPER => 2,
            Play::SCISCORS => 3,
        };

        return Self {
            name: action,
            counter: counter,
            score: score,
        };
    }
}

impl Round {
    fn new(moves: (&str, &str)) -> Self {
        return Self {
            p1: Action::new(moves.0),
            p2: Action::new(moves.1),
        };
    }

    fn resolve(self, part2: bool) -> u32 {
        let mut score = 0;

        if !part2 {
            score += self.p2.score;
            // part 1
            if self.p1.counter == self.p2.name {
                score += WON;
            } else if self.p1.name == self.p2.name {
                score += TIE;
            } else if self.p1.counter == self.p2.name {
                score += LOST;
            }
        } else {
            // part 2
            if self.p2.name == Play::ROCK {
                //must lose
                score += LOST;
                score += match self.p1.name {
                    // pick value for move that loses
                    Play::ROCK => Play::SCISCORS.value(),
                    Play::PAPER => Play::ROCK.value(),
                    Play::SCISCORS => Play::PAPER.value(),
                }
            } else if self.p2.name == Play::PAPER {
                // must tie
                score += TIE;
                score += self.p1.score; // same score as p1 move
            } else if self.p2.name == Play::SCISCORS {
                // must win
                score += WON;
                score += match self.p1.name {
                    // pick value for move that wins against p1
                    Play::ROCK => Play::PAPER.value(),
                    Play::PAPER => Play::SCISCORS.value(),
                    Play::SCISCORS => Play::ROCK.value(),
                }
            }
        }
        return score;
    }
}

pub fn solve() -> String {
    let input_str = util::get_input_lines("inputs/day2");
    let p1 = part1(input_str.clone());
    println!("\tDay 2 Part 1: {:?}", p1);
    let p2 = part2(input_str);
    println!("\tDay 2 Part 2: {:?}", p2);

    return String::from("Day 2");
}

pub fn part1(input_str: Vec<String>) -> u32 {
    let mut score = 0;
    for line in input_str {
        match line.split_once(" ") {
            Some(res) => {
                let round = Round::new(res);
                score += round.resolve(false);
            }
            None => {}
        }
    }

    return score;
}

pub fn part2(input_str: Vec<String>) -> u32 {
    let mut score = 0;
    for line in input_str {
        match line.split_once(" ") {
            Some(res) => {
                let round = Round::new(res);
                score += round.resolve(true);
            }
            None => {}
        }
    }

    return score;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input_str = super::util::get_input_lines("inputs/day2_test");

        let p1 = super::part1(input_str);

        assert_eq!(p1, 15);
    }

    #[test]
    fn part2() {
        let input_str = super::util::get_input_lines("inputs/day2_test");

        let p1 = super::part2(input_str);

        assert_eq!(p1, 12);
    }
}
