use std::collections::HashMap;
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

fn move_south(g: &mut Vec<Vec<Option<bool>>>) -> bool {
    let mut moved = false;
    for r in 0..g.len() - 1 {
        for c in 0..g[r].len() {
            if g[r][c] == Some(true) && g[r + 1][c].is_none() {
                g[r + 1][c] = Some(true);
                g[r][c] = None;
                moved = true;
            }
        }
    }
    moved
}

fn move_west(g: &mut Vec<Vec<Option<bool>>>) -> bool {
    let mut moved = false;
    for r in 0..g.len() {
        for c in 1..g[r].len() {
            if g[r][c] == Some(true) && g[r][c - 1].is_none() {
                g[r][c - 1] = Some(true);
                g[r][c] = None;
                moved = true;
            }
        }
    }
    moved
}

fn move_east(g: &mut Vec<Vec<Option<bool>>>) -> bool {
    let mut moved = false;
    for r in 0..g.len() {
        for c in 0..g[r].len() - 1 {
            if g[r][c] == Some(true) && g[r][c + 1].is_none() {
                g[r][c + 1] = Some(true);
                g[r][c] = None;
                moved = true;
            }
        }
    }
    moved
}

fn cycle(g: &mut Vec<Vec<Option<bool>>>) {
    loop {
        if !move_north(g) {
            break;
        }
    }
    loop {
        if !move_west(g) {
            break;
        }
    }
    loop {
        if !move_south(g) {
            break;
        }
    }
    loop {
        if !move_east(g) {
            break;
        }
    }
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

fn rocks(g: &Vec<Vec<Option<bool>>>) -> Vec<(usize, usize)> {
    let mut rocks = Vec::new();
    for r in 0..g.len() {
        for c in 0..g[r].len() {
            if g[r][c] == Some(true) {
                rocks.push((r, c));
            }
        }
    }
    rocks.sort();
    rocks
}

fn main() {
    let file = File::open("day14.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let g: Vec<Vec<Option<bool>>> = lines
        .map(|r| r.unwrap().chars().map(parse_rock).collect())
        .collect();
    let mut g1 = g.clone();
    loop {
        if !move_north(&mut g1) {
            break;
        }
    }
    let load1 = load(&g1);
    println!("{load1}");
    let mut g2 = g.clone();
    let mut seen: HashMap<Vec<(usize, usize)>, u64> = HashMap::new();
    seen.insert(rocks(&g), 0);
    let mut t = 0;
    let cycle_len = loop {
        cycle(&mut g2);
        t += 1;
        let rocks = rocks(&g2);
        if seen.contains_key(&rocks) {
            break t - seen[&rocks];
        } else {
            seen.insert(rocks, t);
        }
    };
    let needed = (1_000_000_000 - t) % cycle_len;
    for _i in 0..needed {
        cycle(&mut g2);
    }
    let load2 = load(&g2);
    println!("{load2}");
}
