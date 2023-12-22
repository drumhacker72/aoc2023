use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn wrap(a: i64, b: usize) -> usize {
    a.rem_euclid(b as i64) as usize
}

fn main() {
    let file = File::open("day21.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut start = (0, 0);
    let grid = lines
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
    part1(&grid, start);
    part2(&grid, start);
}

fn part1(grid: &Vec<Vec<bool>>, start: (usize, usize)) {
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

fn part2(grid: &Vec<Vec<bool>>, start: (usize, usize)) {
    let h = grid.len();
    let w = grid[0].len();
    // All spots that can be reached in N steps is the union of spots a distance N away,
    // N-2 away, N-4, and so on, since you can always waste 2 moves to go nowhere.
    let mut at_dist = vec![1];
    let mut cursors = HashSet::from([(start.0 as i64, start.1 as i64)]);
    let mut last = cursors.clone();
    // No idea how or why, but visually inspecting the pattern revealed a cycle
    // every 131 steps, where each repetition adds a constant (per-modulo) number of
    // reachable spots.
    // Figure out why the hell this works if time permits.
    const CYCLE: usize = 131;
    for _i in 1..=(CYCLE * 2) {
        let mut next = HashSet::new();
        for &(r, c) in &cursors {
            if grid[wrap(r - 1, h)][wrap(c, w)] && !last.contains(&(r - 1, c)) {
                next.insert((r - 1, c));
            }
            if grid[wrap(r + 1, h)][wrap(c, w)] && !last.contains(&(r + 1, c)) {
                next.insert((r + 1, c));
            }
            if grid[wrap(r, h)][wrap(c - 1, w)] && !last.contains(&(r, c - 1)) {
                next.insert((r, c - 1));
            }
            if grid[wrap(r, h)][wrap(c + 1, w)] && !last.contains(&(r, c + 1)) {
                next.insert((r, c + 1));
            }
        }
        last = cursors;
        cursors = next;
        at_dist.push(cursors.len());
    }
    const END: usize = 26501365;
    for i in (CYCLE * 2 + 1)..=END {
        at_dist.push(at_dist[i - CYCLE] + (at_dist[i - CYCLE] - at_dist[i - CYCLE * 2]));
    }
    let mut sum = 0;
    for i in (1..=END).step_by(2) {
        sum += at_dist[i];
    }
    println!("{sum}");
}
