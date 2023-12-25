use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use rand::prelude::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(s: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (s, (component, _, connecteds)) =
        tuple((alpha1, tag(": "), separated_list1(char(' '), alpha1)))(s)?;
    Ok((s, (component, connecteds)))
}

type Network = BTreeMap<u32, Vec<u32>>;

fn random_edge(network: &Network) -> (u32, u32) {
    let mut rng = thread_rng();
    let a = network.keys().choose(&mut rng).unwrap();
    let b = network[a].iter().choose(&mut rng).unwrap();
    (*a, *b)
}

fn collapse(network: &mut Network, a: u32, b: u32) {
    let mut to_move = network.remove(&b).unwrap();
    for m in &to_move {
        for conn in network.get_mut(m).unwrap() {
            if *conn == b {
                *conn = a;
            }
        }
    }
    network.get_mut(&a).unwrap().append(&mut to_move);
    *network.get_mut(&a).unwrap() = network[&a].iter().copied().filter(|&v| v != a).collect();
}

fn karger(mut network: Network) -> (usize, (u32, u32)) {
    let mut sizes = BTreeMap::new();
    for &k in network.keys() {
        sizes.insert(k, 1);
    }
    while network.len() > 2 {
        let e = random_edge(&network);
        collapse(&mut network, e.0, e.1);
        *sizes.get_mut(&e.0).unwrap() = sizes[&e.0] + sizes.remove(&e.1).unwrap();
    }
    assert!(sizes.len() == 2);
    (
        network.first_key_value().unwrap().1.len(),
        (
            *sizes.first_key_value().unwrap().1,
            *sizes.last_key_value().unwrap().1,
        ),
    )
}

fn main() {
    let file = File::open("day25.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut network: Network = BTreeMap::new();
    let mut keys = BTreeMap::new();
    let mut next = 0;
    for line in lines {
        let (remaining, (a, bs)) = parse_line(&line).unwrap();
        assert!(remaining == "");
        let a = *keys.entry(a.to_string()).or_insert_with(|| {
            next += 1;
            next
        });
        for b in bs {
            let b = *keys.entry(b.to_string()).or_insert_with(|| {
                next += 1;
                next
            });
            network
                .entry(a)
                .and_modify(|e| {
                    e.push(b);
                })
                .or_insert(vec![b]);
            network
                .entry(b)
                .and_modify(|e| {
                    e.push(a);
                })
                .or_insert(vec![a]);
        }
    }
    let sizes = loop {
        let (min_cut, sizes) = karger(network.clone());
        if min_cut == 3 {
            break sizes;
        }
    };
    println!("{}", sizes.0 * sizes.1);
}
