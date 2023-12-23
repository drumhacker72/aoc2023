use std::cmp;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn invert(self) -> Self {
        match self {
            Dir::U => Dir::D,
            Dir::D => Dir::U,
            Dir::L => Dir::R,
            Dir::R => Dir::L,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

impl Tile {
    fn is_slope(&self) -> bool {
        match self {
            Tile::Slope(_) => true,
            _ => false,
        }
    }
}

fn parse_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '^' => Tile::Slope(Dir::U),
        '>' => Tile::Slope(Dir::R),
        'v' => Tile::Slope(Dir::D),
        '<' => Tile::Slope(Dir::L),
        _ => panic!(),
    }
}

fn in_dir(p: (usize, usize), dir: Dir) -> (usize, usize) {
    match dir {
        Dir::U => (p.0 - 1, p.1),
        Dir::D => (p.0 + 1, p.1),
        Dir::L => (p.0, p.1 - 1),
        Dir::R => (p.0, p.1 + 1),
    }
}

fn is_node(grid: &Vec<Vec<Tile>>, p: (usize, usize)) -> bool {
    let mut num_slopes = 0;
    if grid[p.0][p.1] != Tile::Path {
        return false;
    }
    for dir in [Dir::U, Dir::D, Dir::L, Dir::R] {
        let next_p = in_dir(p, dir);
        if grid[next_p.0][next_p.1].is_slope() {
            num_slopes += 1;
        }
    }
    num_slopes > 1
}

fn next_node(grid: &Vec<Vec<Tile>>, p: (usize, usize), dir: Dir) -> ((usize, usize), u32) {
    let mut cursor = in_dir(p, dir);
    let mut last_dir = dir;
    let mut dist = 1;
    while cursor.0 != 0 && cursor.0 != grid.len() - 1 && !is_node(&grid, cursor) {
        for dir in [Dir::U, Dir::D, Dir::L, Dir::R] {
            if dir == last_dir.invert() {
                continue;
            }
            let next_p = in_dir(cursor, dir);
            if grid[next_p.0][next_p.1] != Tile::Forest {
                cursor = next_p;
                last_dir = dir;
                dist += 1;
                break;
            }
        }
    }
    (cursor, dist)
}

fn main() {
    let file = File::open("day23.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let grid: Vec<Vec<Tile>> = lines
        .map(|l| l.chars().map(|c| parse_tile(c)).collect())
        .collect();
    run(&grid, false);
    run(&grid, true);
}

fn run(grid: &Vec<Vec<Tile>>, allow_wrong_way: bool) {
    let start = grid[0].iter().position(|t| *t == Tile::Path).unwrap();
    let start = (0, start);
    let first_node = next_node(&grid, start, Dir::D);
    let mut dists: BTreeMap<(usize, usize), BTreeMap<(usize, usize), u32>> =
        BTreeMap::from([(start, BTreeMap::from([first_node]))]);
    let mut cursors = VecDeque::from([first_node.0]);
    loop {
        if let Some(p) = cursors.pop_front() {
            if p.0 == grid.len() - 1 {
                continue;
            }
            if dists.contains_key(&p) {
                continue;
            }
            dists.insert(p, BTreeMap::new());
            for dir in [Dir::U, Dir::D, Dir::L, Dir::R] {
                let next_p = in_dir(p, dir);
                match grid[next_p.0][next_p.1] {
                    Tile::Slope(allowed_dir) if dir == allowed_dir || allow_wrong_way => {
                        let next = next_node(&grid, p, dir);
                        if next.0 .0 != 0 {
                            dists.get_mut(&p).unwrap().insert(next.0, next.1);
                            cursors.push_back(next.0);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            break;
        }
    }

    let mut cursors = VecDeque::from([(0, start, BTreeSet::new())]);
    let mut max_dist = 0;
    loop {
        if let Some((dist, p, mut path)) = cursors.pop_front() {
            if p.0 == grid.len() - 1 {
                max_dist = cmp::max(max_dist, dist);
                continue;
            }
            path.insert(p);
            for (&next, &delta) in &dists[&p] {
                if path.contains(&next) {
                    continue;
                }
                cursors.push_back((dist + delta, next, path.clone()));
            }
        } else {
            break;
        }
    }
    println!("{max_dist}");
}
