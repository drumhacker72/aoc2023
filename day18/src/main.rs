use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete::{char, i64};
use nom::combinator::{map_res, value};
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

fn parse_dir(s: &str) -> IResult<&str, Dir> {
    alt((
        value(Dir::U, char('U')),
        value(Dir::D, char('D')),
        value(Dir::L, char('L')),
        value(Dir::R, char('R')),
    ))(s)
}

type Color = (i64, Dir);

fn parse_color(s: &str) -> IResult<&str, Color> {
    let (s, (_, n, d)) = tuple((
        char('#'),
        map_res(take(5usize), |n| i64::from_str_radix(n, 16)),
        alt((
            value(Dir::R, char('0')),
            value(Dir::D, char('1')),
            value(Dir::L, char('2')),
            value(Dir::U, char('3')),
        )),
    ))(s)?;
    Ok((s, (n, d)))
}

fn parse_line(s: &str) -> IResult<&str, (Dir, i64, Color)> {
    let (s, (d, _, n, _, c)) = tuple((
        parse_dir,
        char(' '),
        i64,
        char(' '),
        delimited(char('('), parse_color, char(')')),
    ))(s)?;
    Ok((s, (d, n, c)))
}

fn in_dir(pos: (i64, i64), dir: Dir, run: i64) -> (i64, i64) {
    match dir {
        Dir::U => (pos.0, pos.1 + run),
        Dir::D => (pos.0, pos.1 - run),
        Dir::L => (pos.0 - run, pos.1),
        Dir::R => (pos.0 + run, pos.1),
    }
}

fn run(insts: &[(Dir, i64)]) {
    let mut cursor = (0, 0);
    let mut shoelace = 0;
    let mut edge = 0;
    for &(d, n) in insts {
        let last = cursor;
        cursor = in_dir(cursor, d, n);
        edge += n;
        shoelace += last.0 * cursor.1 - last.1 * cursor.0;
    }
    // Shoelace formula gives the area from the middle of each cube, but we want
    // all the way out to the edges. Cubes on the edge are easy, each one will
    // contribute another 1/2 to the count. Cubes on the corner will either
    // contribute 1/4 or 3/4 depending on whether it is inner or outer.
    // I'm sure there exists some formal proof, but it's easy enough to imagine
    // that inner and outer corners will balance out to leave only 4 outer corners
    // unmatched, hence the extra 1 added on.
    println!("{}", shoelace.abs() / 2 + edge / 2 + 1);
}

fn main() {
    let file = File::open("day18.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut insts1 = Vec::new();
    let mut insts2 = Vec::new();
    for line in lines {
        let (remaining, (d1, n1, (n2, d2))) = parse_line(&line).unwrap();
        assert!(remaining == "");
        insts1.push((d1, n1));
        insts2.push((d2, n2));
    }
    run(&insts1);
    run(&insts2);
}
