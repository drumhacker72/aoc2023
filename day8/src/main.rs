use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, line_ending, multispace1};
use nom::combinator::value;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use num::integer::lcm;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
enum Inst {
    L,
    R,
}

fn parse_inst(s: &str) -> IResult<&str, Inst> {
    alt((value(Inst::L, char('L')), value(Inst::R, char('R'))))(s)
}

#[derive(Debug)]
struct Entry<'a> {
    src: &'a str,
    dsts: (&'a str, &'a str),
}

fn parse_entry(s: &str) -> IResult<&str, Entry> {
    let (s, (src, _, dst1, _, dst2, _, _)) = tuple((
        alphanumeric1,
        tag(" = ("),
        alphanumeric1,
        tag(", "),
        alphanumeric1,
        tag(")"),
        line_ending,
    ))(s)?;
    Ok((
        s,
        Entry {
            src,
            dsts: (dst1, dst2),
        },
    ))
}

fn parse_input(s: &str) -> IResult<&str, (Vec<Inst>, Vec<Entry>)> {
    let (s, (insts, _, entries)) = tuple((many1(parse_inst), multispace1, many1(parse_entry)))(s)?;
    Ok((s, (insts, entries)))
}

fn main() {
    let f = fs::read_to_string("day8.txt").unwrap();
    let (remaining, (insts, entries)) = parse_input(&f).unwrap();
    assert!(remaining.is_empty());
    let mut network = HashMap::new();
    for entry in entries {
        network.insert(entry.src, entry.dsts);
    }
    let mut node = "AAA";
    let mut steps = 0;
    for inst in insts.iter().cycle() {
        if node == "ZZZ" {
            break;
        }
        let next = network.get(node).unwrap();
        node = match inst {
            Inst::L => next.0,
            Inst::R => next.1,
        };
        steps += 1;
    }
    println!("{steps}");

    // From experimenting by hand, input seems to be carefully crafted so that the
    // number of iterations until the first "Z" exit for each start node is *exactly*
    // the cycle length.
    // Otherwise this would be a lot more complicated.
    let mut cycles: Vec<u64> = Vec::new();
    for node in network.keys() {
        if !node.ends_with("A") {
            continue;
        }
        let mut cursor: &str = node;
        let mut steps = 0;
        let mut seen: HashMap<(&str, u64), u64> = HashMap::new();
        for inst in insts.iter().cycle() {
            let m = steps % insts.len() as u64;
            if seen.contains_key(&(cursor, m)) {
                let start = *seen.get(&(cursor, m)).unwrap();
                let cycle: u64 = steps - start;
                cycles.push(cycle);
                break;
            }
            seen.insert((cursor, m), steps);
            let next = network.get(cursor).unwrap();
            cursor = match inst {
                Inst::L => next.0,
                Inst::R => next.1,
            };
            steps += 1;
        }
    }
    let lcm: u64 = cycles.iter().fold(1, |acc, &c| lcm(acc, c));
    println!("{lcm}");
}
