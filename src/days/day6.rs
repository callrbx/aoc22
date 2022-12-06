use crate::util;

const DAY_STR: &str = "Day 6";

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/day6");
    let p1 = find_marker(&input_str, 4);
    let p2 = find_marker(&input_str, 14);

    return (
        DAY_STR.to_string(),
        String::from(format!("\n\tPart 1: {}\n\tPart 2: {}", p1, p2)),
    );
}

fn is_marker(marker: &[char], marker_len: usize) -> bool {
    let mut v: Vec<&char> = Vec::new();
    for c in marker {
        if !v.contains(&c) {
            v.push(c);
        }
    }
    return v.len() == marker_len;
}

pub fn find_marker(input_str: &String, marker_len: usize) -> usize {
    let mut marker_idx = marker_len;

    for marker in input_str.chars().collect::<Vec<char>>().windows(marker_len) {
        if is_marker(marker, marker_len) {
            break;
        }
        marker_idx += 1;
    }

    return marker_idx;
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let t1 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let t2 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let t3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let t4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

        assert_eq!(super::find_marker(&t1, 4), 5);
        assert_eq!(super::find_marker(&t2, 4), 6);
        assert_eq!(super::find_marker(&t3, 4), 10);
        assert_eq!(super::find_marker(&t4, 4), 11);
    }

    #[test]
    fn part2() {
        let t1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let t2 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let t3 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let t4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let t5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

        assert_eq!(super::find_marker(&t1, 14), 19);
        assert_eq!(super::find_marker(&t2, 14), 23);
        assert_eq!(super::find_marker(&t3, 14), 23);
        assert_eq!(super::find_marker(&t4, 14), 29);
        assert_eq!(super::find_marker(&t5, 14), 26);
    }
}
