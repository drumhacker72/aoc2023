use nom::character::complete::{char, u32};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (u32, u32, u32);

fn parse_point(s: &str) -> IResult<&str, Point> {
    let (s, (x, _, y, _, z)) = tuple((u32, char(','), u32, char(','), u32))(s)?;
    Ok((s, (x, y, z)))
}

fn parse_brick(s: &str) -> IResult<&str, (Point, Point)> {
    separated_pair(parse_point, char('~'), parse_point)(s)
}

fn cubify(ends: &(Point, Point)) -> Vec<Point> {
    let mut cubes = vec![];
    if ends.0 .1 == ends.1 .1 && ends.0 .2 == ends.1 .2 {
        for x in cmp::min(ends.0 .0, ends.1 .0)..=cmp::max(ends.0 .0, ends.1 .0) {
            cubes.push((x, ends.0 .1, ends.0 .2));
        }
    } else if ends.0 .0 == ends.1 .0 && ends.0 .2 == ends.1 .2 {
        for y in cmp::min(ends.0 .1, ends.1 .1)..=cmp::max(ends.0 .1, ends.1 .1) {
            cubes.push((ends.0 .0, y, ends.0 .2));
        }
    } else if ends.0 .0 == ends.1 .0 && ends.0 .1 == ends.1 .1 {
        for z in cmp::min(ends.0 .2, ends.1 .2)..=cmp::max(ends.0 .2, ends.1 .2) {
            cubes.push((ends.0 .0, ends.0 .1, z));
        }
    } else {
        panic!();
    }
    cubes
}

fn at<T>(grid: &Vec<T>, bounds: Point, p: Point) -> &T {
    &grid[(p.0 + bounds.0 * p.1 + bounds.0 * bounds.1 * p.2) as usize]
}

fn at_mut<T>(grid: &mut Vec<T>, bounds: Point, p: Point) -> &mut T {
    &mut grid[(p.0 + bounds.0 * p.1 + bounds.0 * bounds.1 * p.2) as usize]
}

#[derive(Clone, Debug, PartialEq)]
enum Cube {
    Air,
    Ground,
    Brick(usize),
}

fn down(p: Point) -> Point {
    (p.0, p.1, p.2 - 1)
}

fn supported_by(
    grid: &Vec<Cube>,
    bounds: Point,
    idx: usize,
    ends: &(Point, Point),
) -> (bool, HashSet<usize>) {
    let mut ground = false;
    let mut bricks = HashSet::new();
    for cube in cubify(ends) {
        match *at(grid, bounds, down(cube)) {
            Cube::Ground => {
                ground = true;
            }
            Cube::Brick(b) => {
                if b != idx {
                    bricks.insert(b);
                }
            }
            Cube::Air => {}
        }
    }
    (ground, bricks)
}

fn drop(grid: &mut Vec<Cube>, bounds: Point, idx: usize, ends: &mut (Point, Point)) {
    for cube in cubify(ends) {
        assert!(*at(grid, bounds, cube) == Cube::Brick(idx));
        *at_mut(grid, bounds, cube) = Cube::Air;
    }
    ends.0 .2 -= 1;
    ends.1 .2 -= 1;
    for cube in cubify(ends) {
        assert!(*at(grid, bounds, cube) == Cube::Air);
        *at_mut(grid, bounds, cube) = Cube::Brick(idx);
    }
}

fn main() {
    let file = File::open("day22.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut bricks = vec![];
    for line in lines {
        let (remaining, ends) = parse_brick(&line).unwrap();
        assert!(remaining == "");
        bricks.push(ends);
    }
    let mut bounds = (0, 0, 0);
    for ends in bricks.iter() {
        bounds.0 = cmp::max(bounds.0, 1 + cmp::max(ends.0 .0, ends.1 .0));
        bounds.1 = cmp::max(bounds.1, 1 + cmp::max(ends.0 .1, ends.1 .1));
        bounds.2 = cmp::max(bounds.2, 1 + cmp::max(ends.0 .2, ends.1 .2));
    }
    let mut grid = vec![Cube::Air; (bounds.0 * bounds.1 * bounds.2) as usize];
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            *at_mut(&mut grid, bounds, (x, y, 0)) = Cube::Ground;
        }
    }
    for (i, ends) in bricks.iter().enumerate() {
        for cube in cubify(ends) {
            assert!(*at(&grid, bounds, cube) == Cube::Air);
            *at_mut(&mut grid, bounds, cube) = Cube::Brick(i);
        }
    }
    loop {
        let mut changed = false;
        for (i, ends) in bricks.iter_mut().enumerate() {
            if supported_by(&grid, bounds, i, ends) == (false, HashSet::new()) {
                drop(&mut grid, bounds, i, ends);
                changed = true;
                break;
            }
        }
        if !changed {
            break;
        }
    }

    let mut num_safe = 0;
    for i in 0..bricks.len() {
        let mut safe = true;
        for (j, ends) in bricks.iter().enumerate() {
            if i == j {
                continue;
            }
            let (g, bs) = supported_by(&grid, bounds, j, ends);
            if !g && bs == HashSet::from([i]) {
                safe = false;
                break;
            }
        }
        if safe {
            num_safe += 1;
        }
    }
    println!("{num_safe}");
}
