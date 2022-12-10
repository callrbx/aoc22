use crate::util;

const DAY_STR: &str = "Day 10";

fn draw_output(pixels: Vec<bool>, onc: char, offc: char) -> String {
    let mut output = String::new();
    for line in pixels.chunks(40) {
        let display = line
            .iter()
            .map(|p| if *p { onc } else { offc })
            .collect::<String>();

        output.push('\n');
        output.push_str(&display);
    }

    return output;
}

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/day10");
    let (p1, p2) = parts1_2(&input_str);

    return (
        DAY_STR.to_string(),
        String::from(format!("\n\tPart 1: {:?}\n\tPart 2: {}", p1, p2)),
    );
}

pub fn parts1_2(input_str: &String) -> (i32, String) {
    let mut states = Vec::new();
    let mut pixels: Vec<bool> = Vec::new();
    let mut draw_pos = 0;
    let mut reg = 1;
    states.push(reg); // account for cycle 0

    for line in input_str.split("\n") {
        match line {
            "noop" => {
                states.push(reg);
                pixels.push(draw_pos >= reg - 1 && draw_pos <= reg + 1); // draw pixel
                draw_pos += 1; // increase pos
                draw_pos = draw_pos % 40; // wrap
            }
            addi if line.starts_with("addx") => {
                states.push(reg); // first cycle
                pixels.push(draw_pos >= reg - 1 && draw_pos <= reg + 1); // draw pixel
                draw_pos += 1; // increase pos
                draw_pos = draw_pos % 40; // wrap
                states.push(reg); // second cycle
                pixels.push(draw_pos >= reg - 1 && draw_pos <= reg + 1); // draw pixel
                draw_pos += 1; // increase pos
                draw_pos = draw_pos % 40; // wrap
                reg += addi.split_once(' ').unwrap().1.parse::<i32>().unwrap() // at end of 2nd cycle, value increases
            }
            _ => {}
        }
    }

    let mut strength = 0;
    let mut idx = 20;

    while idx < states.len() {
        strength += states[idx] * idx as i32;
        idx += 40;
    }

    return (strength, draw_output(pixels, '*', ' '));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input_str = super::util::get_input("inputs/day10_test");

        // part 2 requires visual inspection; not testing
        let (p1, _) = super::parts1_2(&input_str);

        assert_eq!(p1, 13140);
    }
}
