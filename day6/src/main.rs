use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter;

fn parse_int_list(line: &str) -> Vec<u64> {
    let mut tokens = line.split_whitespace();
    tokens.next();
    tokens.map(|s| s.parse().unwrap()).collect()
}

fn parse_wide_int(line: &str) -> u64 {
    let mut tokens = line.split_whitespace();
    tokens.next();
    tokens.collect::<Vec<_>>().join("").parse().unwrap()
}

fn race_distance(button_time: u64, total_time: u64) -> u64
{
    button_time * (total_time - button_time)
}

fn part1() {
    let file = File::open("day6.txt").unwrap();
    let mut lines = BufReader::new(file).lines();
    let times = parse_int_list(&lines.next().unwrap().unwrap());
    let distances = parse_int_list(&lines.next().unwrap().unwrap());
    let races = iter::zip(times, distances);
    let mut race_ways = Vec::new();
    for (time, distance) in races {
        let mut ways = 0;
        for b in 0..=time {
            if race_distance(b, time) > distance { ways += 1 }
        }
        race_ways.push(ways);
    }
    println!("{}", race_ways.iter().fold(1, |acc, w| acc * w));
}

fn part2() {
    let file = File::open("day6.txt").unwrap();
    let mut lines = BufReader::new(file).lines();
    let time = parse_wide_int(&lines.next().unwrap().unwrap());
    let distance = parse_wide_int(&lines.next().unwrap().unwrap());
    let mut ways = 0;
    for b in 0..=time {
        if race_distance(b, time) > distance { ways += 1 }
    }
    println!("{ways}");
}

fn main() {
    part1();
    part2();
}
