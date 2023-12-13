use std::cmp;
use std::fs;

fn rows_eq<T: PartialEq>(g: &[T], width: usize, r1: usize, r2: usize) -> bool {
    g[r1*width..(r1+1)*width] == g[r2*width..(r2+1)*width]
}

fn test_h<T: PartialEq>(g: &[T], width: usize, before_row: usize) -> bool {
    let num_rows = cmp::min(before_row, g.len()/width - before_row);
    assert!(num_rows != 0);
    for i in 0..num_rows {
        let r1 = before_row - i - 1;
        let r2 = before_row + i;
        if !rows_eq(g, width, r1, r2) { return false; }
    }
    true
}

fn try_h<T: PartialEq>(g: &[T], width: usize) -> Option<usize> {
    for i in 1..g.len()/width {
        if test_h(g, width, i) { return Some(i) }
    }
    None
}

fn cols_eq<T: PartialEq>(g: &[T], width: usize, c1: usize, c2: usize) -> bool {
    for r in 0..g.len()/width {
        if g[r*width+c1] != g[r*width+c2] { return false; }
    }
    true
}

fn test_v<T: PartialEq>(g: &[T], width: usize, before_col: usize) -> bool {
    let num_cols = cmp::min(before_col, width - before_col);
    assert!(num_cols != 0);
    for i in 0..num_cols {
        let c1 = before_col - i - 1;
        let c2 = before_col + i;
        if !cols_eq(g, width, c1, c2) { return false; }
    }
    true
}

fn try_v<T: PartialEq>(g: &[T], width: usize) -> Option<usize> {
    for i in 1..width {
        if test_v(g, width, i) { return Some(i) }
    }
    None
}

fn main() {
    let f = fs::read_to_string("day13.txt").unwrap();
    let mut total = 0;
    for pattern in f.split("\n\n") {
        let rows: Vec<Vec<char>> = pattern.split('\n').map(|r| r.chars().collect()).collect();
        let width = rows[0].len();
        let g = rows.join(&[] as &[char]);
        let h = try_h(&g, width);
        let v = try_v(&g, width);
        assert!(h.is_some() ^ v.is_some());
        if let Some(r) = h { total += 100 * r; }
        if let Some(c) = v { total += c; }
    }
    println!("{total}");
}
