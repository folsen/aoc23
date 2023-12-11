use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.expect("line exists")).collect()
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_line(line: &String) -> Game {
    let gid = Regex::new(r"Game ([0-9]+):").unwrap();
    let cap = gid.captures(line).unwrap().get(1).unwrap();
    let id_match: String = cap.as_str().to_owned();
    let id = id_match.parse().unwrap();
    let rounds = line[cap.end()+1..].split(";").map(|s| parse_round(s)).collect();
    Game {id: id, rounds: rounds}
}

fn parse_round(round: &str) -> Round {
    let r = Regex::new(r"([0-9]+) red").unwrap();
    let g = Regex::new(r"([0-9]+) green").unwrap();
    let b = Regex::new(r"([0-9]+) blue").unwrap();
    Round {
        red: find_or_0(r, round),
        green: find_or_0(g, round),
        blue: find_or_0(b, round)
    }
}

fn find_or_0(re: Regex, inp: &str) -> u32 {
    let Some(caps) = re.captures(inp) else {
        return 0;
    };
    caps[1].parse().unwrap_or(0)
}

fn valid_game(game: &Game, tr: u32, tg: u32, tb: u32) -> bool {
    game.rounds.iter().all(|r| r.red <= tr && r.green <= tg && r.blue <= tb)
}

fn main() {
    let lines = read_input("input.txt");    
    let games: Vec<Game> = lines.iter().map(|l| parse_line(l)).collect();
    // Determine which games would have been possible if the bag had been 
    // loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. 
    // What is the sum of the IDs of those games?
    let part1: usize = games.iter().filter(|g| valid_game(&g, 12, 13, 14)).map(|g| g.id).sum();
    println!("{:?}", part1);
}
