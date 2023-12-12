use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, multispace1, space1, u64};
use nom::multi::{separated_list1, many1};
use nom::sequence::{separated_pair, tuple};
use std::cmp;
use std::collections::HashMap;
use std::fs;

struct MapLine {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

struct Map<'a> {
    src: &'a str,
    dst: &'a str,
    lines: Vec<MapLine>,
}

impl Map<'_> {
    fn get(&self, src: u64) -> u64 {
        for line in self.lines.iter() {
            if line.src_start <= src && src < line.src_start + line.len {
                return src - line.src_start + line.dst_start;
            }
        }
        return src;
    }
}

struct Almanac<'a> {
    maps: HashMap<&'a str, Map<'a>>,
}

impl Almanac<'_> {
    fn seed_to_location(&self, seed_id: u64) -> u64 {
        let mut category = "seed";
        let mut id = seed_id;
        while category != "location" {
            let m = self.maps.get(category).unwrap();
            category = m.dst;
            id = m.get(id);
        }
        id
    }
}

fn parse_map_line(s: &str) -> IResult<&str, MapLine> {
    let (s, (dst_start, _, src_start, _, len, _)) = tuple((
        u64,
        space1,
        u64,
        space1,
        u64,
        line_ending,
    ))(s)?;
    Ok((s, MapLine { dst_start, src_start, len }))
}

fn parse_map(s: &str) -> IResult<&str, Map> {
    let (s, ((src, dst), _, _, lines)) = tuple((
        separated_pair(alpha1, tag("-to-"), alpha1),
        tag(" map:"),
        line_ending,
        many1(parse_map_line),
    ))(s)?;
    Ok((s, Map { src, dst, lines }))
}

fn parse_input(s: &str) -> IResult<&str, (Vec<u64>, Almanac)> {
    let (s, (_, _, seeds, _, maps)) = tuple((
        tag("seeds:"),
        space1,
        separated_list1(space1, u64),
        multispace1,
        separated_list1(multispace1, parse_map),
    ))(s)?;
    let mut src_maps: HashMap<&str, Map> = HashMap::new();
    for map in maps {
        src_maps.insert(map.src, map);
    }
    let almanac = Almanac { maps: src_maps };
    Ok((s, (seeds, almanac)))
}

fn main() {
    let f = fs::read_to_string("day5.txt").unwrap();
    let (remaining, (seeds, almanac)) = parse_input(&f).unwrap();
    assert!(remaining.is_empty());
    let lowest = seeds.iter().map(|&n| almanac.seed_to_location(n)).min().unwrap();
    println!("{lowest}");

    let mut lowest_chunked = u64::MAX;
    for chunk in seeds.chunks(2) {
        let &[start, len] = chunk else { panic!() };
        for i in start..(start+len) {
            lowest_chunked = cmp::min(lowest_chunked, almanac.seed_to_location(i));
        }
    }
    println!("{lowest_chunked}");
}
