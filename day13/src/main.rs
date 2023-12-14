use std::cmp;
use std::fs;

fn row_mismatches<T: PartialEq>(g: &[T], width: usize, r1: usize, r2: usize) -> usize {
    (0..width)
        .filter(|c| g[r1 * width + c] != g[r2 * width + c])
        .count()
}

fn test_h<T: PartialEq>(g: &[T], width: usize, before_row: usize) -> usize {
    let num_rows = cmp::min(before_row, g.len() / width - before_row);
    assert!(num_rows != 0);
    let mut mismatches = 0;
    for i in 0..num_rows {
        let r1 = before_row - i - 1;
        let r2 = before_row + i;
        mismatches += row_mismatches(g, width, r1, r2);
    }
    mismatches
}

fn try_h<T: PartialEq>(g: &[T], width: usize, mismatches: usize) -> Option<usize> {
    for i in 1..g.len() / width {
        if test_h(g, width, i) == mismatches {
            return Some(i);
        }
    }
    None
}

fn col_mismatches<T: PartialEq>(g: &[T], width: usize, c1: usize, c2: usize) -> usize {
    (0..(g.len() / width))
        .filter(|r| g[r * width + c1] != g[r * width + c2])
        .count()
}

fn test_v<T: PartialEq>(g: &[T], width: usize, before_col: usize) -> usize {
    let num_cols = cmp::min(before_col, width - before_col);
    assert!(num_cols != 0);
    let mut mismatches = 0;
    for i in 0..num_cols {
        let c1 = before_col - i - 1;
        let c2 = before_col + i;
        mismatches += col_mismatches(g, width, c1, c2);
    }
    mismatches
}

fn try_v<T: PartialEq>(g: &[T], width: usize, mismatches: usize) -> Option<usize> {
    for i in 1..width {
        if test_v(g, width, i) == mismatches {
            return Some(i);
        }
    }
    None
}

fn main() {
    let f = fs::read_to_string("day13.txt").unwrap();
    let mut total1 = 0;
    let mut total2 = 0;
    for pattern in f.split("\n\n") {
        let rows: Vec<Vec<char>> = pattern.split('\n').map(|r| r.chars().collect()).collect();
        let width = rows[0].len();
        let g = rows.join(&[] as &[char]);
        let h = try_h(&g, width, 0);
        let v = try_v(&g, width, 0);
        assert!(h.is_some() ^ v.is_some());
        if let Some(r) = h {
            total1 += 100 * r;
        }
        if let Some(c) = v {
            total1 += c;
        }

        let h = try_h(&g, width, 1);
        let v = try_v(&g, width, 1);
        assert!(h.is_some() ^ v.is_some());
        if let Some(r) = h {
            total2 += 100 * r;
        }
        if let Some(c) = v {
            total2 += c;
        }
    }
    println!("{total1}");
    println!("{total2}");
}
