use std::fs::File;
use std::io::{BufRead, BufReader};

fn condition_report(springs: &Vec<bool>) -> Vec<u32> {
    let mut report = Vec::new();
    let mut run = 0;
    for &spring in springs {
        if spring {
            run += 1;
        } else if run != 0 {
            report.push(run);
            run = 0;
        }
    }
    if run != 0 {
        report.push(run);
    }
    report
}

struct Arrangements<'a> {
    springs: &'a Vec<Option<bool>>,
    perm: u32,
    end: u32,
}

impl Iterator for Arrangements<'_> {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.perm == self.end {
            return None;
        }
        let mut a = Vec::new();
        let mut mask = 1;
        for s in self.springs {
            match s {
                None => {
                    a.push(self.perm & mask != 0);
                    mask <<= 1;
                }
                Some(b) => {
                    a.push(*b);
                }
            }
        }
        self.perm += 1;
        Some(a)
    }
}

fn arrangements(springs: &Vec<Option<bool>>) -> Arrangements {
    Arrangements {
        springs,
        perm: 0,
        end: 1 << springs.iter().filter(|s| s.is_none()).count(),
    }
}

fn main() {
    let file = File::open("day12.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut total = 0;
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

        for arr in arrangements(&springs) {
            if condition_report(&arr) == report {
                total += 1;
            }
        }
    }
    println!("{total}");
}
