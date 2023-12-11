use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    multi::separated_list1,
    sequence::{delimited, tuple},
};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, BufRead},
};

#[derive(Debug)]
struct Card {
    id: u32,
    winners: HashSet<u32>,
    numbers: Vec<u32>,
}

fn parse_number_list(s: &str) -> IResult<&str, Vec<u32>> {
    delimited(space0, separated_list1(space1, u32), space0)(s)
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (s, (_, _, id, _, winners, _, numbers)) = tuple((
        tag("Card"),
        space1,
        u32,
        tag(":"),
        parse_number_list,
        tag("|"),
        parse_number_list,
    ))(s)?;
    Ok((s, Card { id, winners: HashSet::from_iter(winners.iter().cloned()), numbers }))
}

fn score(card: &Card) -> u32 {
    let mut score = 0;
    for n in card.numbers.iter() {
        if card.winners.contains(n) {
            if score == 0 { score = 1; } else { score *= 2; }
        }
    }
    score
}

fn main() {
    let file = File::open("day4.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut scores = 0;
    for line in lines {
        let l = line.unwrap();
        let (remaining, card) = parse_card(&l).unwrap();
        assert!(remaining.is_empty());
        scores += score(&card);
    }
    println!("{}", scores);
}
