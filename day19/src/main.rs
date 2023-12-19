use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, one_of, u64};
use nom::combinator::{map, value};
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Cat {
    X,
    M,
    A,
    S,
}

fn parse_cat(s: &str) -> IResult<&str, Cat> {
    alt((
        value(Cat::X, char('x')),
        value(Cat::M, char('m')),
        value(Cat::A, char('a')),
        value(Cat::S, char('s')),
    ))(s)
}

#[derive(Debug)]
enum Cond {
    Lt(Cat, u64),
    Gt(Cat, u64),
}

fn parse_cond(s: &str) -> IResult<&str, Cond> {
    let (s, (cat, op, val)) = tuple((parse_cat, one_of("<>"), u64))(s)?;
    let cond = match op {
        '<' => Cond::Lt(cat, val),
        '>' => Cond::Gt(cat, val),
        _ => panic!(),
    };
    Ok((s, cond))
}

#[derive(Clone, Debug)]
enum Dest {
    Accepted,
    Rejected,
    Workflow(String),
}

fn parse_dest(s: &str) -> IResult<&str, Dest> {
    alt((
        value(Dest::Accepted, char('A')),
        value(Dest::Rejected, char('R')),
        map(alpha1, |n: &str| Dest::Workflow(n.to_string())),
    ))(s)
}

type Rule = (Cond, Dest);

fn parse_rule(s: &str) -> IResult<&str, Rule> {
    separated_pair(parse_cond, char(':'), parse_dest)(s)
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    fallback: Dest,
}

fn parse_workflow(s: &str) -> IResult<&str, (&str, Workflow)> {
    let (s, (name, _, rules, _, fallback, _)) = tuple((
        alpha1,
        char('{'),
        separated_list0(char(','), parse_rule),
        char(','),
        parse_dest,
        char('}'),
    ))(s)?;
    Ok((s, (name, Workflow { rules, fallback })))
}

type Ratings = HashMap<Cat, u64>;

fn parse_ratings(s: &str) -> IResult<&str, Ratings> {
    let (s, (_, x, _, m, _, a, _, shiny, _)) = tuple((
        tag("{x="),
        u64,
        tag(",m="),
        u64,
        tag(",a="),
        u64,
        tag(",s="),
        u64,
        char('}'),
    ))(s)?;
    Ok((
        s,
        HashMap::from([(Cat::X, x), (Cat::M, m), (Cat::A, a), (Cat::S, shiny)]),
    ))
}

fn test_cond(c: &Cond, r: &Ratings) -> bool {
    match c {
        Cond::Lt(cat, val) => r[cat] < *val,
        Cond::Gt(cat, val) => r[cat] > *val,
    }
}

fn process<'a>(w: &'a Workflow, r: &Ratings) -> &'a Dest {
    for (c, d) in &w.rules {
        if test_cond(&c, r) {
            return d;
        }
    }
    return &w.fallback;
}

fn update(ranges: &mut HashMap<Cat, (u64, u64)>, c: &Cond, invert: bool) {
    match c {
        Cond::Lt(cat, val) => {
            if !invert {
                ranges.get_mut(cat).unwrap().1 = cmp::min(ranges[cat].1, *val - 1);
            } else {
                ranges.get_mut(cat).unwrap().0 = cmp::max(ranges[cat].0, *val);
            }
        }
        Cond::Gt(cat, val) => {
            if !invert {
                ranges.get_mut(cat).unwrap().0 = cmp::max(ranges[cat].0, *val + 1);
            } else {
                ranges.get_mut(cat).unwrap().1 = cmp::min(ranges[cat].1, *val);
            }
        }
    }
}

fn analyze(
    workflows: &HashMap<String, Workflow>,
    cursor: &str,
    mut ranges: HashMap<Cat, (u64, u64)>,
) -> Vec<HashMap<Cat, (u64, u64)>> {
    let w = &workflows[cursor];
    let mut new_ranges = Vec::new();
    for (c, d) in &w.rules {
        match d {
            Dest::Accepted => {
                let mut fin = ranges.clone();
                update(&mut fin, c, false);
                new_ranges.push(fin);
                update(&mut ranges, c, true);
            }
            Dest::Rejected => {
                update(&mut ranges, c, true);
            }
            Dest::Workflow(next) => {
                let mut branch = ranges.clone();
                update(&mut branch, c, false);
                new_ranges.extend(analyze(workflows, next, branch));
                update(&mut ranges, c, true);
            }
        }
    }
    match &w.fallback {
        Dest::Accepted => {
            new_ranges.push(ranges);
        }
        Dest::Rejected => {}
        Dest::Workflow(next) => {
            new_ranges.extend(analyze(workflows, &next, ranges));
        }
    }
    new_ranges
}

fn possibilities(ranges: &[(u64, u64)]) -> u64 {
    ranges.iter().fold(
        1,
        |acc, (min, max)| {
            if min <= max {
                acc * (max - min + 1)
            } else {
                0
            }
        },
    )
}

fn main() {
    let file = File::open("day19.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut iter = lines.into_iter();
    let mut workflows = HashMap::new();
    loop {
        let line = iter.next().unwrap();
        if line == "" {
            break;
        }
        let (remaining, (name, workflow)) = parse_workflow(&line).unwrap();
        assert!(remaining == "");
        workflows.insert(name.to_string(), workflow);
    }
    let mut total_score = 0;
    loop {
        let line = iter.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        let (remaining, ratings) = parse_ratings(&line).unwrap();
        assert!(remaining == "");
        let mut cursor = "in";
        let score = loop {
            match process(&workflows[cursor], &ratings) {
                Dest::Accepted => {
                    break ratings.values().fold(0, |acc, v| acc + v);
                }
                Dest::Rejected => {
                    break 0;
                }
                Dest::Workflow(n) => {
                    cursor = n;
                }
            }
        };
        total_score += score;
    }
    println!("{total_score}");

    let init = HashMap::from([
        (Cat::X, (1, 4000)),
        (Cat::M, (1, 4000)),
        (Cat::A, (1, 4000)),
        (Cat::S, (1, 4000)),
    ]);
    let range_sets = analyze(&workflows, "in", init);
    let ps = range_sets.iter().fold(0, |acc, rs| {
        let ranges = Vec::from_iter(rs.values().copied());
        acc + possibilities(&ranges[..])
    });
    println!("{ps}");
}
