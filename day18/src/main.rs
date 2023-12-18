use nom::IResult;
use nom::branch::alt;
use nom::character::complete::{char, hex_digit1, u32};
use nom::combinator::{map_res, value};
use nom::sequence::{delimited, tuple};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
enum Dir {
    U, D, L, R,
}

fn parse_dir(s: &str) -> IResult<&str, Dir> {
    alt((
        value(Dir::U, char('U')),
        value(Dir::D, char('D')),
        value(Dir::L, char('L')),
        value(Dir::R, char('R')),
    ))(s)
}

type Color = u32;

fn parse_color(s: &str) -> IResult<&str, Color> {
    map_res(
        tuple((char('#'), hex_digit1)),
        |(_, n)| u32::from_str_radix(n, 16)
    )(s)
}

fn parse_line(s: &str) -> IResult<&str, (Dir, u32, Color)> {
    let (s, (d, _, n, _, c)) = tuple((
        parse_dir,
        char(' '),
        u32,
        char(' '),
        delimited(char('('), parse_color, char(')')),
    ))(s)?;
    Ok((s, (d, n, c)))
}

fn in_dir(pos: (i32, i32), dir: Dir, run: i32) -> (i32, i32) {
    match dir {
        Dir::U => (pos.0, pos.1 + run),
        Dir::D => (pos.0, pos.1 - run),
        Dir::L => (pos.0 - run, pos.1),
        Dir::R => (pos.0 + run, pos.1),
    }
}

fn main() {
    let file = File::open("day18.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut cursor = (0, 0);
    let mut last = (0, 0);
    let mut shoelace = 0;
    let mut edge = 0;
    let mut seen = HashSet::new();
    seen.insert((0, 0));
    for line in lines {
        let (remaining, (d, n, c)) = parse_line(&line).unwrap();
        assert!(remaining == "");
        last = cursor;
        for _i in 0..n {
            cursor = in_dir(cursor, d, 1);
            seen.insert(cursor);
            println!("{:?}", cursor);
        }
        shoelace += last.0 * cursor.1 - last.1 * cursor.0;
    }
    //TODO: justify this
    println!("{}", shoelace.abs() / 2 + seen.len() as i32 / 2 + 1);
}
