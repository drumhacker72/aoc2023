use nom::character::complete::{char, i64, space1};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_vec3(s: &str) -> IResult<&str, Vec3> {
    let (s, (x, _, _, y, _, _, z)) =
        tuple((i64, char(','), space1, i64, char(','), space1, i64))(s)?;
    Ok((s, Vec3 { x, y, z }))
}

fn parse_line(s: &str) -> IResult<&str, (Vec3, Vec3)> {
    separated_pair(parse_vec3, delimited(space1, char('@'), space1), parse_vec3)(s)
}

fn intersection_2d(p_a: &Vec3, v_a: &Vec3, p_b: &Vec3, v_b: &Vec3) -> Option<(f64, f64)> {
    // x = pa.x + va.x * t1 = pb.x + vb.x * t2
    // y = pa.y + va.y * t1 = pb.y + vb.y * t2

    // vb.y (pa.x + va.x * t1) - vb.x (pa.y + va.y * t1) = vb.y * pb.x - vb.x - pb.y
    // t1 = (vb.y (pb.x - pa.x) - vb.x (pb.y - pa.y)) / (vb.y * va.x - vb.x * va.y)
    if v_b.y * v_a.x == v_b.x * v_a.y {
        return None;
    }
    let vax = v_a.x as f64;
    let vay = v_a.y as f64;
    let vbx = v_b.x as f64;
    let vby = v_b.y as f64;
    let dx = (p_b.x - p_a.x) as f64;
    let dy = (p_b.y - p_a.y) as f64;
    let t1 = (vby * dx - vbx * dy) / (vby * vax - vbx * vay);
    if t1 < 0.0 {
        return None;
    }
    let t2 = (vax * t1 - dx) / vbx;
    if t2 < 0.0 {
        return None;
    }
    let x = p_a.x as f64 + vax * t1;
    let y = p_a.y as f64 + vay * t1;
    Some((x, y))
}

fn main() {
    let file = File::open("day24.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let hailstones: Vec<(Vec3, Vec3)> = lines
        .map(|line| {
            let (remaining, (p, v)) = parse_line(&line).unwrap();
            assert!(remaining == "");
            (p, v)
        })
        .collect();
    let mut count = 0;
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;
    for i in 0..hailstones.len() {
        for j in 0..i {
            let a = &hailstones[i];
            let b = &hailstones[j];
            if let Some((x, y)) = intersection_2d(&a.0, &a.1, &b.0, &b.1) {
                if MIN <= x && x <= MAX && MIN <= y && y <= MAX {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");
}
