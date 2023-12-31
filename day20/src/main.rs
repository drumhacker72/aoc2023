use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char};
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcast,
    Untyped,
}

impl Module {
    fn is_conjunction(&self) -> bool {
        match self {
            Module::Conjunction(_) => true,
            _ => false,
        }
    }
}

fn parse_flip_flop(s: &str) -> IResult<&str, (&str, Module)> {
    let (s, (_, name)) = tuple((char('%'), alpha1))(s)?;
    Ok((s, (name, Module::FlipFlop(false))))
}

fn parse_conjunction(s: &str) -> IResult<&str, (&str, Module)> {
    let (s, (_, name)) = tuple((char('&'), alpha1))(s)?;
    Ok((s, (name, Module::Conjunction(HashMap::new()))))
}

fn parse_module(s: &str) -> IResult<&str, (&str, Module)> {
    alt((
        parse_flip_flop,
        parse_conjunction,
        value(("broadcaster", Module::Broadcast), tag("broadcaster")),
    ))(s)
}

fn parse_line(s: &str) -> IResult<&str, (&str, Module, Vec<&str>)> {
    let (s, ((name, module), _, destinations)) = tuple((
        parse_module,
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    ))(s)?;
    Ok((s, (name, module, destinations)))
}

fn send_pulse(module: &mut Module, pulse: Pulse, source: &str) -> Option<Pulse> {
    match module {
        Module::FlipFlop(is_on) => match pulse {
            Pulse::High => None,
            Pulse::Low => {
                if !*is_on {
                    *is_on = true;
                    Some(Pulse::High)
                } else {
                    *is_on = false;
                    Some(Pulse::Low)
                }
            }
        },
        Module::Conjunction(inputs) => {
            inputs.insert(source.to_string(), pulse);
            if inputs.values().all(|&p| p == Pulse::High) {
                Some(Pulse::Low)
            } else {
                Some(Pulse::High)
            }
        }
        Module::Broadcast => Some(pulse),
        Module::Untyped => None,
    }
}

type Network = HashMap<String, (Module, Vec<String>)>;

fn push_button1(network: &mut Network) -> (u64, u64) {
    let mut q = VecDeque::from([("broadcaster".to_string(), Pulse::Low, "button".to_string())]);
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    while !q.is_empty() {
        let (name, pulse, source) = q.pop_front().unwrap();
        match pulse {
            Pulse::Low => {
                low_pulses += 1;
            }
            Pulse::High => {
                high_pulses += 1;
            }
        }
        let (module, destinations) = network.get_mut(&name).unwrap();
        if let Some(result) = send_pulse(module, pulse, &source) {
            for dest in destinations {
                q.push_back((dest.to_string(), result, name.clone()));
            }
        }
    }
    (low_pulses, high_pulses)
}

fn push_button2(network: &mut Network, watch_high: &str) -> Vec<String> {
    let mut watched = Vec::new();
    let mut q = VecDeque::from([("broadcaster".to_string(), Pulse::Low, "button".to_string())]);
    while !q.is_empty() {
        let (name, pulse, source) = q.pop_front().unwrap();
        if name == watch_high && pulse == Pulse::High {
            watched.push(source.to_string());
        }
        let (module, destinations) = network.get_mut(&name).unwrap();
        if let Some(result) = send_pulse(module, pulse, &source) {
            for dest in destinations {
                q.push_back((dest.to_string(), result, name.clone()));
            }
        }
    }
    watched
}

fn main() {
    let file = File::open("day20.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let mut network: Network = HashMap::new();
    let mut sources: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let (remaining, (name, module, destinations)) = parse_line(&line).unwrap();
        assert!(remaining == "");
        network.insert(
            name.to_string(),
            (module, destinations.iter().map(|d| d.to_string()).collect()),
        );
        for dest in destinations {
            sources
                .entry(dest.to_string())
                .and_modify(|e| e.push(name.to_string()))
                .or_insert(vec![name.to_string()]);
        }
    }
    for (name, this_sources) in &sources {
        network
            .entry(name.clone())
            .and_modify(|e| match e {
                (Module::Conjunction(inputs), _) => {
                    for this_source in this_sources {
                        inputs.insert(this_source.clone(), Pulse::Low);
                    }
                }
                _ => {}
            })
            .or_insert((Module::Untyped, vec![]));
    }
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut network1 = network.clone();
    for _i in 0..1000 {
        let (l, h) = push_button1(&mut network1);
        low_pulses += l;
        high_pulses += h;
    }
    println!("{}", low_pulses * high_pulses);

    // Looking for low pulse to rx...
    let penult = &sources["rx"];
    assert!(penult.len() == 1);
    let penult = &penult[0];
    assert!(network[penult].0.is_conjunction());
    // means looking for the cycle with all high pulses to the penultimate
    // module (a conjunction for the given input).
    let mut cycles = HashMap::new();
    for source in &sources[penult] {
        cycles.insert(source.to_string(), None);
    }
    let mut network2 = network.clone();
    let mut presses = 0;
    while cycles.values().any(|c| c.is_none()) {
        presses += 1;
        for source in push_button2(&mut network2, &penult) {
            cycles.insert(source, Some(presses));
        }
    }
    println!(
        "{}",
        cycles.values().fold(1u64, |acc, c| lcm(acc, c.unwrap()))
    );
}
