use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq)]
struct Card(char, bool);

impl Card {
    fn strength(&self) -> u32 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if self.1 {
                    1
                } else {
                    11
                }
            }
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
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

fn type_from_groups(groups: &Vec<u32>) -> HandType {
    match groups.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if groups.contains(&4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if groups.contains(&3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!(),
    }
}

fn make_groups(cards: [char; 5], replace_joker: char) -> HashMap<char, u32> {
    let mut groups: HashMap<char, u32> = HashMap::new();
    for card in cards {
        let card = if card == 'J' { replace_joker } else { card };
        groups.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    groups
}

fn parse_line(line: &str, jokers: bool) -> (Hand, u32) {
    let mut iter = line.chars();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    let d = iter.next().unwrap();
    let e = iter.next().unwrap();
    assert!(iter.next() == Some(' '));
    let bid = iter.as_str().parse().unwrap();

    let cards = [a, b, c, d, e];
    let hand_type = if jokers {
        let mut best = HandType::HighCard;
        for replace in ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'] {
            let groups = make_groups(cards, replace);
            let hand_type = type_from_groups(&groups.values().copied().collect());
            best = cmp::max(best, hand_type);
        }
        best
    } else {
        let groups = make_groups(cards, 'J');
        type_from_groups(&groups.values().copied().collect())
    };
    let cards = [
        Card(a, jokers),
        Card(b, jokers),
        Card(c, jokers),
        Card(d, jokers),
        Card(e, jokers),
    ];
    (Hand { hand_type, cards }, bid)
}

fn part1() {
    let file = File::open("day7.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut hand_bids: Vec<(Hand, u32)> = lines.map(|l| parse_line(&l.unwrap(), false)).collect();
    hand_bids.sort();
    let mut winnings = 0;
    for i in 0..hand_bids.len() {
        winnings += hand_bids[i].1 * (i as u32 + 1);
    }
    println!("{}", winnings);
}

fn part2() {
    let file = File::open("day7.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut hand_bids: Vec<(Hand, u32)> = lines.map(|l| parse_line(&l.unwrap(), true)).collect();
    hand_bids.sort();
    let mut winnings = 0;
    for i in 0..hand_bids.len() {
        winnings += hand_bids[i].1 * (i as u32 + 1);
    }
    println!("{}", winnings);
}

fn main() {
    part1();
    part2();
}
