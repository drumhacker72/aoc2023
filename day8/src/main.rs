use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, line_ending, multispace1};
use nom::combinator::value;
use nom::multi::many1;
use nom::sequence::tuple;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
enum Inst {
    L,
    R,
}

fn parse_inst(s: &str) -> IResult<&str, Inst> {
    alt((
        value(Inst::L, char('L')),
        value(Inst::R, char('R')),
    ))(s)
}

#[derive(Debug)]
struct Entry<'a> {
    src: &'a str,
    dsts: (&'a str, &'a str),
}

fn parse_entry(s: &str) -> IResult<&str, Entry> {
    let (s, (src, _, dst1, _, dst2, _, _)) = tuple((
        alpha1,
        tag(" = ("),
        alpha1,
        tag(", "),
        alpha1,
        tag(")"),
        line_ending,
    ))(s)?;
    Ok((s, Entry { src, dsts: (dst1, dst2) }))
}

fn parse_input(s: &str) -> IResult<&str, (Vec<Inst>, Vec<Entry>)> {
    let (s, (insts, _, entries)) = tuple((
        many1(parse_inst),
        multispace1,
        many1(parse_entry),
    ))(s)?;
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
        if node == "ZZZ" { break; }
        let next = network.get(node).unwrap();
        node = match inst {
            Inst::L => next.0,
            Inst::R => next.1,
        };
        steps += 1;
    }
    println!("{steps}");
}
