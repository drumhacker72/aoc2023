use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::value,
    multi::separated_list1,
};
use std::{
    fs::File,
    io::{BufReader, BufRead},
    ops::Add,
};

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
        Counts { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

const EMPTY: Counts = Counts { r: 0, g: 0, b: 0 };

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
    let (s, _) = tag("Game ")(s)?;
    let (s, id) = u32(s)?;
    let (s, _) = tag(": ")(s)?;
    let (s, rounds) = separated_list1(tag("; "), parse_round)(s)?;
    Ok((s, Game { id, rounds }))
}

fn main() {
    let file = File::open("day2.txt").unwrap();
    let lines = BufReader::new(file).lines();
    const LIMITS: Counts = Counts { r: 12, g: 13, b: 14 };
    let mut good_ids = 0;
    for line in lines {
        let l = line.unwrap();
        let (remaining, game) = parse_game(&l).unwrap();
        assert!(remaining.is_empty());
        let is_good = game.rounds.iter().all(|x| x.is_within(LIMITS));
        if is_good { good_ids += game.id; }
    }
    println!("{}", good_ids);
}
