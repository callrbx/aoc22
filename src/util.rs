use std::fs::File;
use std::io::{BufReader, Read};

pub fn get_input(filename: &str) -> String {
    let mut str = String::new();
    let file = File::open(filename).expect("Error in reading file");
    let mut reader = BufReader::new(file);
    reader
        .read_to_string(&mut str)
        .expect("Unable to read line");
    return str;
}
