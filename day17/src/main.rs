use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn inverse(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W,
        }
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

fn main() {
    let file = File::open("day17.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let grid: Vec<Vec<u32>> = lines.map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), (0, 0), None));
    let mut seen = HashMap::new();
    let heat_loss = loop {
        if let Some((Reverse(dist), p, run)) = pq.pop() {
            if p.0 == grid.len()-1 && p.1 == grid[p.0].len()-1 {
                break dist;
            }
            if let Some(&prev_dist) = seen.get(&(p, run)) {
                if dist >= prev_dist { continue; }
            } else { seen.insert((p, run), dist); }
            for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
                if let Some(new_p) = in_dir(&grid, p, dir) {
                    let new_dist = dist + grid[new_p.0][new_p.1];
                    if let Some(new_run) = match run {
                        Some((d, 3)) if d == dir => None,
                        Some((d, r)) if d == dir => Some((dir, r+1)),
                        Some((d, _)) if d == dir.inverse() => None,
                        _ => Some((dir, 1)),
                    } {
                        pq.push((Reverse(new_dist), new_p, Some(new_run)));
                    }
                }
            }
        } else { panic!(); }
    };
    println!("{heat_loss}");
}
