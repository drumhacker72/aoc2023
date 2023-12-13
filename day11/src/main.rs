use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<bool>>;

fn is_col_empty(g: &Grid, col: usize) -> bool {
    for r in g {
        if r[col] {
            return false;
        }
    }
    true
}

fn main() {
    let file = File::open("day11.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let grid: Grid = lines
        .map(|l| l.unwrap().chars().map(|c| c == '#').collect())
        .collect();
    let mut row_ts = HashMap::new();
    let mut row_t = 0;
    for row in 0..grid.len() {
        if !grid[row].iter().any(|&x| x) {
            row_t += 1;
        } else {
            row_ts.insert(row, row_t);
        }
        row_t += 1;
    }
    let mut col_ts = HashMap::new();
    let mut col_t = 0;
    for col in 0..grid[0].len() {
        if is_col_empty(&grid, col) {
            col_t += 1;
        } else {
            col_ts.insert(col, col_t);
        }
        col_t += 1;
    }
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] {
                galaxies.push((*row_ts.get(&row).unwrap(), *col_ts.get(&col).unwrap()));
            }
        }
    }
    let mut distances = 0;
    for i in 0..galaxies.len() {
        let (r1, c1) = galaxies[i];
        for j in 0..i {
            let (r2, c2) = galaxies[j];
            distances += r1.abs_diff(r2) + c1.abs_diff(c2);
        }
    }
    println!("{distances}");
}
