use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Eq, PartialEq)]
struct Card(char);

impl Card {
    fn strength(&self) -> u32 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            d => d.to_digit(10).expect("card should be a digit"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    groups: Vec<u32>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type(hand: &Hand) -> HandType {
    match hand.groups.len() {
        1 => HandType::FiveOfAKind,
        2 => if hand.groups.contains(&4) { HandType::FourOfAKind } else { HandType::FullHouse },
        3 => if hand.groups.contains(&3) { HandType::ThreeOfAKind } else { HandType::TwoPair },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!(),
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match hand_type(self).cmp(&hand_type(other)) {
            Ordering::Equal => { self.cards.cmp(&other.cards) }
            o => o
        }
    }
}

fn parse_line(line: &str) -> (Hand, u32) {
    let mut iter = line.chars();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    let d = iter.next().unwrap();
    let e = iter.next().unwrap();
    assert!(iter.next() == Some(' '));
    let bid = iter.as_str().parse().unwrap();

    let mut groups: HashMap<char, u32> = HashMap::new();
    for card in [a, b, c, d, e] {
        groups.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    let cards = [Card(a), Card(b), Card(c), Card(d), Card(e)];
    let groups: Vec<u32> = groups.values().copied().collect();
    (Hand { cards, groups }, bid)
}

fn main() {
    let file = File::open("day7.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut hand_bids: Vec<(Hand, u32)> = lines.map(|l| parse_line(&l.unwrap())).collect();
    hand_bids.sort();
    let mut winnings = 0;
    for i in 0..hand_bids.len() {
        winnings += hand_bids[i].1 * (i as u32 + 1);
    }
    println!("{}", winnings);
}
