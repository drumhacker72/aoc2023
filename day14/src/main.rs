use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_rock(c: char) -> Option<bool> {
    match c {
        'O' => Some(true),
        '#' => Some(false),
        '.' => None,
        _ => panic!(),
    }
}

fn move_north(g: &mut Vec<Vec<Option<bool>>>) -> bool {
    let mut moved = false;
    for r in 1..g.len() {
        for c in 0..g[r].len() {
            if g[r][c] == Some(true) && g[r - 1][c].is_none() {
                g[r - 1][c] = Some(true);
                g[r][c] = None;
                moved = true;
            }
        }
    }
    moved
}

fn load(g: &Vec<Vec<Option<bool>>>) -> usize {
    let mut load = 0;
    for r in 0..g.len() {
        for c in 0..g[r].len() {
            if g[r][c] == Some(true) {
                load += g.len() - r;
            }
        }
    }
    load
}

fn main() {
    let file = File::open("day14.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut g: Vec<Vec<Option<bool>>> = lines
        .map(|r| r.unwrap().chars().map(parse_rock).collect())
        .collect();
    loop {
        if !move_north(&mut g) {
            break;
        }
    }
    let load = load(&g);
    println!("{load}");
}
