use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn can_match(springs: &[Option<bool>], run: usize) -> bool {
    if springs.len() < run {
        return false;
    }
    for i in 0..run {
        if springs[i] == Some(false) {
            return false;
        }
    }
    springs.len() == run || springs[run] != Some(true)
}

fn arrangements(springs: &[Option<bool>], report: &[u32]) -> u64 {
    thread_local!(static MEMO: RefCell<HashMap<(Vec<Option<bool>>, Vec<u32>), u64>> = RefCell::new(HashMap::new()));
    MEMO.with(|memo| {
        if let Some(v) = memo.borrow().get(&(springs.to_vec(), report.to_vec())) {
            return *v;
        }
        if report.is_empty() {
            if !springs.iter().any(|&s| s == Some(true)) {
                return 1;
            } else {
                return 0;
            }
        }
        if springs.is_empty() {
            return 0;
        }
        let v = match springs[0] {
            Some(false) => arrangements(&springs[1..], report),
            Some(true) => {
                let run = report[0] as usize;
                if can_match(springs, run) {
                    if springs.len() == run {
                        arrangements(&[], &report[1..])
                    } else {
                        arrangements(&springs[run + 1..], &report[1..])
                    }
                } else {
                    0
                }
            }
            None => {
                let mut springs1 = vec![Some(false)];
                springs1.append(&mut springs[1..].to_vec());
                let mut springs2 = vec![Some(true)];
                springs2.append(&mut springs[1..].to_vec());
                arrangements(&springs1, report) + arrangements(&springs2, report)
            }
        };
        memo.borrow_mut()
            .insert((springs.to_vec(), report.to_vec()), v);
        v
    })
}

fn main() {
    let file = File::open("day12.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut total1 = 0;
    let mut total2 = 0;
    for line in lines {
        let l = line.unwrap();
        let (springs, report) = l.split_once(' ').unwrap();
        let springs: Vec<Option<bool>> = springs
            .chars()
            .map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                '?' => None,
                _ => panic!(),
            })
            .collect();
        let report: Vec<u32> = report.split(',').map(|n| n.parse().unwrap()).collect();
        total1 += arrangements(&springs, &report);
        let x: [&[Option<bool>]; 5] = [&springs, &springs, &springs, &springs, &springs];
        let springs: Vec<Option<bool>> = x.join(&None);
        let report = report.repeat(5);
        total2 += arrangements(&springs, &report);
    }
    println!("{total1}");
    println!("{total2}");
}
