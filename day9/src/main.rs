use std::fs::File;
use std::io::{BufReader, BufRead};

fn delta(vs: &Vec<i32>) -> Vec<i32> {
    let mut ds = Vec::new();
    for i in 0..vs.len()-1 {
        ds.push(vs[i+1] - vs[i]);
    }
    ds
}

fn is_zeroes(vs: &Vec<i32>) -> bool {
    vs.iter().all(|&v| v == 0)
}

fn extrapolate(vs: &Vec<i32>) -> i32 {
    if is_zeroes(&vs) { 0 }
    else {
        let ds = delta(&vs);
        let next_d = extrapolate(&ds);
        vs.last().unwrap() + next_d
    }
}

fn main() {
    let file = File::open("day9.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut sum = 0;
    for line in lines {
        let values: Vec<i32> = line.unwrap().split(' ').map(|v| v.parse().unwrap()).collect();
        sum += extrapolate(&values);
    }
    println!("{sum}");
}
