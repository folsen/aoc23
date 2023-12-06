use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Nums = Vec<Vec<char>>;

fn read_input(file_name: &str) -> Nums {
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    reader.lines().enumerate().map(|(_, l)| l.expect("line exists").chars().collect()).collect()
}

fn filter_num(nums: Nums) -> Nums {
    let mut new_nums = Vec::new();
    for line in nums.iter() {
        new_nums.push(line.iter().filter(|c| c.is_numeric()).cloned().collect());
    }
    new_nums
}


fn scanreplace_left(str: &String) -> String {
    let mut start= "".to_owned();
    let mut end = str.clone();
    while end.len() > 0 {
        start.push(end.remove(0));
        start = replacedigit(&start);
    }
    start
}

fn replacedigit(str: &String) -> String {
    str
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

fn pickfl(nums: Nums) -> Vec<u32> {
    let mut res = Vec::new();
    for (i,v) in nums.iter().enumerate() {
        print!("{}: {:?} = ",i, v);
        let f = v.first().unwrap().to_digit(10).unwrap();
        let l = v.last().unwrap().to_digit(10).unwrap();
        res.push(f*10 + l);
        println!("{}", f*10+l);
    }
    res
}

fn main() {
    let mut nums = read_input("input.txt");
    nums = filter_num(nums);
    let ints = pickfl(nums);
    println!("{:?}", ints.iter().sum::<u32>());
}

#[test]
fn testdouble() {
    assert_eq!(scanreplace_left(&"twone".to_owned()), "2ne");
    assert_eq!(scanreplace_left(&"twone5eightwo".to_owned()), "2ne58wo");
    assert_eq!(scanreplace_left(&"4nineeightseven2".to_owned()), "49872");
}

// part 1 54940
// incorrect part 2 54194
