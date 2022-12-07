use crate::util;

const DAY_STR: &str = "Day X";

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/dayX");
    let p1 = part1(&input_str);
    let p2 = part2(&input_str);

    return (
        DAY_STR.to_string(),
        String::from(format!("\n\tPart 1: {:?}\n\tPart 2: {:?}", p1, p2)),
    );
}

pub fn part1(input_str: &String) -> u32 {
    return 0;
}

pub fn part2(input_str: &String) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input_str = super::util::get_input("inputs/dayX_test");

        let p1 = super::part1(&input_str);

        assert_eq!(p1, 0);
    }

    #[test]
    fn part2() {
        let input_str = super::util::get_input("inputs/dayX_test");

        let p2 = super::part2(&input_str);

        assert_eq!(p2, 0);
    }
}
