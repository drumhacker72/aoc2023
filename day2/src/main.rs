use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct Counts {
    r: u32,
    g: u32,
    b: u32,
}

impl Counts {
    fn is_within(self, other: Self) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }
}

impl Add for Counts {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Counts {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

const EMPTY: Counts = Counts { r: 0, g: 0, b: 0 };

fn max_each(a: Counts, b: Counts) -> Counts {
    Counts {
        r: cmp::max(a.r, b.r),
        g: cmp::max(a.g, b.g),
        b: cmp::max(a.b, b.b),
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Counts>,
}

fn parse_counted_color(s: &str) -> IResult<&str, Counts> {
    let (s, count) = u32(s)?;
    let (s, _) = tag(" ")(s)?;
    alt((
        value(Counts { r: count, ..EMPTY }, tag("red")),
        value(Counts { g: count, ..EMPTY }, tag("green")),
        value(Counts { b: count, ..EMPTY }, tag("blue")),
    ))(s)
}

fn parse_round(s: &str) -> IResult<&str, Counts> {
    let (s, counts) = separated_list1(tag(", "), parse_counted_color)(s)?;
    Ok((s, counts.iter().fold(EMPTY, |acc, &x| acc + x)))
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    let (s, (_, id, _, rounds)) = tuple((
        tag("Game "),
        u32,
        tag(": "),
        separated_list1(tag("; "), parse_round),
    ))(s)?;
    Ok((s, Game { id, rounds }))
}

fn main() {
    let file = File::open("day2.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    const LIMITS: Counts = Counts {
        r: 12,
        g: 13,
        b: 14,
    };
    let mut good_ids = 0;
    let mut power_sum = 0;
    for line in lines {
        let (remaining, game) = parse_game(&line).unwrap();
        assert!(remaining.is_empty());
        let is_good = game.rounds.iter().all(|x| x.is_within(LIMITS));
        if is_good {
            good_ids += game.id;
        }

        let min_needed = game.rounds.iter().fold(EMPTY, |acc, &x| max_each(acc, x));
        let power = min_needed.r * min_needed.g * min_needed.b;
        power_sum += power;
    }
    println!("{}", good_ids);
    println!("{}", power_sum);
}
