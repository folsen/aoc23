use std::fs::File;
use std::io::{prelude::*, BufReader};

type Nums = Vec<String>;

fn read_input(file_name: &str) -> Nums {
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.expect("line exists")).collect()
}

fn main() {
    let nums = read_input("input.txt");    
    let mut res: Vec<u32> = Vec::new();
    for v in nums.iter() {
        let mut d = Vec::new();
        for (i, c) in v.chars().enumerate() {
            if c.is_numeric() {
                d.push(c.to_digit(10).unwrap());
            } else {
                for (j, n) in vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {
                    if v[i..].starts_with(n) {
                        d.push(j as u32 +1);
                    }
                }
            }
        }
        if d.len() == 1 {
            res.push(d[0]*10+d[0]);
        } else {
            res.push(d[0]*10+d[d.len()-1]);
        }
    }
    println!("{}", res.iter().sum::<u32>());
}
