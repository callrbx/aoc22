use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn get_input(filename: &str) -> String {
    let mut input: String = String::new();
    for line in get_input_lines(filename) {
        if let Ok(l) = line {
            input.push_str(&l);
            input.push_str("\n");
        }
    }
    return input;
}

pub fn get_input_lines(filename: &str) -> std::io::Lines<BufReader<File>> {
    if let Ok(lines) = read_lines(filename) {
        return lines;
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
