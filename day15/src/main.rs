use nom::branch::alt;
use nom::character::complete::{alpha1, char, u32};
use nom::combinator::value;
use nom::sequence::tuple;
use nom::IResult;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Op {
    Remove,
    Set(u32),
}

fn parse_set(s: &str) -> IResult<&str, Op> {
    let (s, _) = char('=')(s)?;
    let (s, value) = u32(s)?;
    Ok((s, Op::Set(value)))
}

fn parse_op(s: &str) -> IResult<&str, Op> {
    alt((value(Op::Remove, char('-')), parse_set))(s)
}

#[derive(Debug)]
struct Inst {
    label: String,
    op: Op,
}

fn parse_inst(s: &str) -> IResult<&str, Inst> {
    let (s, (label, op)) = tuple((alpha1, parse_op))(s)?;
    Ok((
        s,
        Inst {
            label: label.to_string(),
            op,
        },
    ))
}

fn hash(s: &str) -> u8 {
    let mut v = 0;
    for c in s.chars() {
        v = ((v as u32 + c as u32) * 17) as u8;
    }
    v
}

type Box = Vec<(String, u32)>;

fn remove(b: &mut Box, label: &str) {
    for i in 0..b.len() {
        if b[i].0 == label {
            b.remove(i);
            return;
        }
    }
}

fn set(b: &mut Box, label: &str, value: u32) {
    for i in 0..b.len() {
        if b[i].0 == label {
            b[i].1 = value;
            return;
        }
    }
    b.push((label.to_string(), value));
}

fn power(boxes: &[Box]) -> u32 {
    let mut power = 0;
    for b in 0..boxes.len() {
        for slot in 0..boxes[b].len() {
            power += (b as u32 + 1) * (slot as u32 + 1) * boxes[b][slot].1;
        }
    }
    power
}

fn main() {
    let f = fs::read_to_string("day15.txt").unwrap();
    let f = f.lines().collect::<Vec<_>>().join("");
    let mut checksum: u32 = 0;
    const EMPTY: Box = Vec::new();
    let mut boxes: [Box; 256] = [EMPTY; 256];
    for step in f.split(',') {
        checksum += hash(step) as u32;
        let (remaining, inst) = parse_inst(step).unwrap();
        assert!(remaining == "");
        let h = hash(&inst.label) as usize;
        match inst.op {
            Op::Remove => remove(&mut boxes[h], &inst.label),
            Op::Set(value) => set(&mut boxes[h], &inst.label, value),
        }
    }
    println!("{checksum}");
    println!("{}", power(&boxes));
}
