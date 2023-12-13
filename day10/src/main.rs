use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn invert(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pipe(Dir, Dir);

type Grid = Vec<Vec<Option<Pipe>>>;

fn turn(d: Dir, p: Pipe) -> Option<Dir> {
    if p.0 == d.invert() {
        Some(p.1)
    } else if p.1 == d.invert() {
        Some(p.0)
    } else {
        None
    }
}

fn dmove(g: &Grid, d: Dir, p: (usize, usize)) -> Option<(usize, usize)> {
    match d {
        Dir::N => {
            if p.0 == 0 {
                None
            } else {
                Some((p.0 - 1, p.1))
            }
        }
        Dir::S => {
            if p.0 == g.len() - 1 {
                None
            } else {
                Some((p.0 + 1, p.1))
            }
        }
        Dir::E => {
            if p.1 == g[p.0].len() - 1 {
                None
            } else {
                Some((p.0, p.1 + 1))
            }
        }
        Dir::W => {
            if p.1 == 0 {
                None
            } else {
                Some((p.0, p.1 - 1))
            }
        }
    }
}

fn get(g: &Grid, p: (usize, usize)) -> Option<Pipe> {
    g[p.0][p.1]
}

fn parse_pipe(c: char) -> Option<Pipe> {
    match c {
        '|' => Some(Pipe(Dir::N, Dir::S)),
        '-' => Some(Pipe(Dir::E, Dir::W)),
        'L' => Some(Pipe(Dir::N, Dir::E)),
        'J' => Some(Pipe(Dir::N, Dir::W)),
        '7' => Some(Pipe(Dir::S, Dir::W)),
        'F' => Some(Pipe(Dir::S, Dir::E)),
        '.' => None,
        _ => panic!(),
    }
}

fn starts(g: &Grid, p: (usize, usize)) -> Vec<((usize, usize), Dir)> {
    let mut starts = Vec::new();
    for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
        if let Some(pp) = dmove(g, d, p) {
            if let Some(pipe) = get(&g, pp) {
                if let Some(dd) = turn(d, pipe) {
                    starts.push((pp, dd));
                }
            }
        }
    }
    assert!(starts.len() == 2);
    starts
}

fn main() {
    let file = File::open("day10.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut grid = Vec::new();
    let mut start = (0, 0);
    for (row, line) in lines.enumerate() {
        let l = line.unwrap();
        let pipes: Vec<Option<Pipe>> = l
            .chars()
            .enumerate()
            .map(|(col, c)| {
                if c == 'S' {
                    start = (row, col);
                    None
                } else {
                    parse_pipe(c)
                }
            })
            .collect();
        grid.push(pipes);
    }
    let mut cursors = starts(&grid, start);
    let mut dist = 1;
    while cursors[0].0 != cursors[1].0 {
        cursors = cursors
            .iter()
            .map(|&(p, d)| {
                let pp = dmove(&grid, d, p).unwrap();
                let pipe = get(&grid, pp).unwrap();
                let dd = turn(d, pipe).unwrap();
                (pp, dd)
            })
            .collect();
        dist += 1;
    }
    println!("{dist}");

    let mut last = start;
    let (mut cur, mut d) = starts(&grid, start)[0];
    let mut shoelace: isize = 0;
    loop {
        shoelace += (last.0 as isize) * (cur.1 as isize) - (cur.0 as isize) * (last.1 as isize);
        last = cur;
        cur = dmove(&grid, d, cur).unwrap();
        if cur == start {
            break;
        }
        let pipe = get(&grid, cur).unwrap();
        d = turn(d, pipe).unwrap();
    }
    shoelace += (last.0 as isize) * (cur.1 as isize) - (cur.0 as isize) * (last.1 as isize);
    println!("{}", (shoelace.abs() - (dist - 1) * 2) / 2);
}
