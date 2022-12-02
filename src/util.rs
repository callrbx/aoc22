use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_input(filename: &str) -> String {
    let input = get_input_lines(filename);
    return input.join("\n");
}

pub fn get_input_lines(filename: &str) -> Vec<String> {
    if let Ok(lines) = read_lines(filename) {
        return lines.map(|l| l.expect("Could not parse line")).collect();
    } else {
        eprint!("Failed to read {}", filename);
        std::process::exit(1);
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
