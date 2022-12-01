use crate::util;

pub fn solve() {
    // File hosts must exist in current path before this produces output

    let input_str = util::get_input("inputs/day1");
    let mut group_cals = part1(input_str);

    let p2 = part2(&mut group_cals);
    let p1 = group_cals.iter().max().unwrap();

    println!("Day 1 Part 1: {:?}", p1);
    println!("Day 1 Part 2: {:?}", p2);
}

pub fn part1(input_str: String) -> Vec<u32> {
    let mut group_cals: Vec<u32> = Vec::new();

    let carriers = input_str.split("\n\n");

    for group in carriers {
        let mut group_cal = 0;
        for l in group.split("\n") {
            match l.parse::<u32>() {
                Ok(n) => group_cal += n,
                Err(_) => {} // probably blank line
            }
        }
        group_cals.push(group_cal);
    }

    return group_cals;
}

pub fn part2(group_cals: &mut Vec<u32>) -> u32 {
    group_cals.sort_by(|a, b| a.cmp(b).reverse());
    group_cals.truncate(3);

    return group_cals.iter().sum();
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "\
            1000\n\
            2000\n\
            3000\n\
            \n\
            4000\n\
            \n\
            5000\n\
            6000\n\
            \n\
            7000\n\
            8000\n\
            9000\n\
            \n\
            10000\n";

    #[test]
    fn day1_test() {
        let mut group_cals = super::part1(INPUT.to_string());

        let p2 = super::part2(&mut group_cals);
        let p1 = *group_cals.iter().max().unwrap();

        assert_eq!(p1, 24000);
        assert_eq!(p2, 45000);
    }
}
