use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    Ok((
        s,
        Card {
            id,
            winners: HashSet::from_iter(winners.iter().cloned()),
            numbers,
        },
    ))
}

fn num_winners(card: &Card) -> u32 {
    card.numbers
        .iter()
        .filter(|n| card.winners.contains(n))
        .count() as u32
}

fn score(card: &Card) -> u32 {
    match num_winners(card) {
        0 => 0,
        c => 1 << (c - 1),
    }
}

fn main() {
    let file = File::open("day4.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut scores = 0;
    let mut copies: HashMap<u32, u32> = HashMap::new();
    let mut total_cards = 0;
    for line in lines {
        let (remaining, card) = parse_card(&line).unwrap();
        assert!(remaining.is_empty());
        scores += score(&card);

        let count = 1 + copies.get(&card.id).unwrap_or(&0);
        total_cards += count;
        let winners = num_winners(&card);
        for id in card.id + 1..=card.id + winners {
            copies
                .entry(id)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }
        copies.remove(&card.id);
    }
    println!("{}", scores);
    println!("{}", total_cards);
}
