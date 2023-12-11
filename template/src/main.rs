use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.expect("line exists")).collect()
}

fn main() {
    let _ = read_input("input.txt");    
}
