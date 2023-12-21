use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day21.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut start = (0, 0);
    let grid: Vec<Vec<bool>> = lines
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, tile)| match tile {
                    'S' => {
                        start = (r, c);
                        true
                    }
                    '.' => true,
                    '#' => false,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let mut cursors = HashSet::from([start]);
    for _i in 0..64 {
        let mut next = HashSet::new();
        for (r, c) in cursors {
            if r > 0 && grid[r - 1][c] {
                next.insert((r - 1, c));
            }
            if r < grid.len() - 1 && grid[r + 1][c] {
                next.insert((r + 1, c));
            }
            if c > 0 && grid[r][c - 1] {
                next.insert((r, c - 1));
            }
            if c < grid[r].len() - 1 && grid[r][c + 1] {
                next.insert((r, c + 1));
            }
        }
        cursors = next;
    }
    println!("{}", cursors.len());
}
