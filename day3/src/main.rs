use std::cmp;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Tile {
   Digit(u32),
   Symbol(char),
   Empty,
}

type Grid = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
struct Loc {
    row: usize,
    col: usize,
}

fn tile_at(g: &Grid, loc: Loc) -> Tile {
    g[loc.row][loc.col]
}

fn parse_tile(c: char) -> Tile {
    if let Some(d) = c.to_digit(10) { Tile::Digit(d) }
    else if c == '.' { Tile::Empty }
    else { Tile::Symbol(c) }
}

#[derive(Clone, Copy, Debug)]
struct Number {
    value: u32,
    loc: Loc,
    len: usize,
}

fn find_numbers(g: &Grid) -> Vec<Number> {
    let mut nums = Vec::new();
    for row in 0..g.len() {
        let mut col = 0;
        while col < g[row].len() {
            let loc = Loc { row, col };
            col += 1;
            if let Tile::Digit(d) = tile_at(g, loc) {
                let mut value = d;
                while let Tile::Digit(d) = tile_at(g, Loc { row, col }) {
                    col += 1;
                    value = value * 10 + d;
                    if col == g[row].len() { break; }
                }
                nums.push(Number { value, loc, len: col - loc.col });
            }
        }
    }
    nums
}

fn is_near_symbol(g: &Grid, n: Number) -> bool {
    for row in n.loc.row.checked_sub(1).unwrap_or(0)..cmp::min(g.len(), n.loc.row + 2) {
        for col in n.loc.col.checked_sub(1).unwrap_or(0)..cmp::min(g[row].len(), n.loc.col + n.len + 1) {
            if let Tile::Symbol(_) = g[row][col] { return true; }
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = fs::read_to_string("day3.txt")?;
    let g: Grid = f.lines().map(|line|
            line.chars().map(parse_tile).collect()
        ).collect();
    let mut part_numbers = 0;
    for &n in find_numbers(&g).iter() {
        if is_near_symbol(&g, n) { part_numbers += n.value; }
    }
    println!("{}", part_numbers);
    Ok(())
}
