use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    N,
    S,
    W,
    E,
}

fn handle(tile: char, dir: Dir) -> Vec<Dir> {
    match tile {
        '.' => vec![dir],
        '/' => vec![match dir {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::W => Dir::S,
            Dir::E => Dir::N,
        }],
        '\\' => vec![match dir {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::W => Dir::N,
            Dir::E => Dir::S,
        }],
        '-' => match dir {
            Dir::N | Dir::S => vec![Dir::W, Dir::E],
            Dir::W => vec![Dir::W],
            Dir::E => vec![Dir::E],
        },
        '|' => match dir {
            Dir::N => vec![Dir::N],
            Dir::S => vec![Dir::S],
            Dir::W | Dir::E => vec![Dir::N, Dir::S],
        },
        _ => panic!(),
    }
}

fn in_dir<T>(grid: &Vec<Vec<T>>, pos: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
    match dir {
        Dir::N => {
            if pos.0 == 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
        Dir::S => {
            if pos.0 == grid.len() - 1 {
                None
            } else {
                Some((pos.0 + 1, pos.1))
            }
        }
        Dir::W => {
            if pos.1 == 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
        Dir::E => {
            if pos.1 == grid[pos.0].len() - 1 {
                None
            } else {
                Some((pos.0, pos.1 + 1))
            }
        }
    }
}

fn num_energized(grid: &Vec<Vec<char>>, pos: (usize, usize), dir: Dir) -> usize {
    let mut cursors = vec![(pos, dir)];
    let mut seen = HashSet::new();
    while !cursors.is_empty() {
        let (pos, dir) = cursors.pop().unwrap();
        if seen.contains(&(pos, dir)) {
            continue;
        } else {
            seen.insert((pos, dir));
        }
        for split_dir in handle(grid[pos.0][pos.1], dir) {
            if let Some(split_pos) = in_dir(&grid, pos, split_dir) {
                cursors.push((split_pos, split_dir));
            }
        }
    }
    let mut energized = HashSet::new();
    for (pos, _) in seen {
        energized.insert(pos);
    }
    energized.len()
}

fn main() {
    let file = File::open("day16.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let grid: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let energized = num_energized(&grid, (0, 0), Dir::E);
    println!("{energized}");

    let mut best = energized;
    let mut starts = Vec::new();
    for r in 0..grid.len() {
        if r != 0 {
            starts.push(((r, 0), Dir::E));
        }
        starts.push(((r, grid[r].len() - 1), Dir::W));
    }
    for c in 0..grid[0].len() {
        starts.push(((0, c), Dir::S));
        starts.push(((grid.len() - 1, c), Dir::N));
    }
    for (pos, dir) in starts {
        let e = num_energized(&grid, pos, dir);
        best = cmp::max(best, e);
    }
    println!("{best}");
}
