use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("day1.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut total1 = 0;
    let mut total2 = 0;
    for line in lines {
        let l = line.unwrap();
        total1 += calibration(&l, false);
        total2 += calibration(&l, true);
    }
    println!("{total1}");
    println!("{total2}");
}

fn calibration(line: &str, allow_spelled: bool) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    let mut cursor: &str = &line;
    while !cursor.is_empty() {
        let d = front_digit(&cursor, allow_spelled);
        if d.is_some() {
            if first_digit.is_none() { first_digit = d; }
            last_digit = d;
        }
        cursor = &cursor[1..];
    }
    first_digit.unwrap() * 10 + last_digit.unwrap()
}

fn front_digit(cursor: &str, allow_spelled: bool) -> Option<u32> {
    cursor.chars().next()?.to_digit(10).or_else(||
        if !allow_spelled { None }
        else if cursor.starts_with("one") { Some(1) }
        else if cursor.starts_with("two") { Some(2) }
        else if cursor.starts_with("three") { Some(3) }
        else if cursor.starts_with("four") { Some(4) }
        else if cursor.starts_with("five") { Some(5) }
        else if cursor.starts_with("six") { Some(6) }
        else if cursor.starts_with("seven") { Some(7) }
        else if cursor.starts_with("eight") { Some(8) }
        else if cursor.starts_with("nine") { Some(9) }
        else { None }
    )
}
