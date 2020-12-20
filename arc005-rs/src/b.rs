#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

fn main() {
    let (x, y, w) = read_tuple!(usize, usize, String);

    let c = read_vec(9, || read_str());

    let (dx, dy) = w.chars().fold((0i64, 0i64), |(dx, dy), c| match c {
        'R' => (dx + 1, dy),
        'L' => (dx - 1, dy),
        'U' => (dx, dy - 1),
        'D' => (dx, dy + 1),
        _ => unreachable!(),
    });

    let dx = if dx == -1 && x == 1 {
        1
    } else if dx == 1 && x == 9 {
        -1
    } else {
        dx
    };

    let dy = if dy == -1 && y == 1 {
        1
    } else if dy == 1 && y == 9 {
        -1
    } else {
        dy
    };

    let ans = successors(Some((x - 1, y - 1, dx, dy)), |&(x, y, dx, dy)| {
        let nx = (x as i64 + dx) as usize;
        let ny = (y as i64 + dy) as usize;

        if nx == 0 || nx == 8 {
            if ny == 0 || ny == 8 {
                Some((nx, ny, -dx, -dy))
            } else {
                Some((nx, ny, -dx, dy))
            }
        } else {
            if ny == 0 || ny == 8 {
                Some((nx, ny, dx, -dy))
            } else {
                Some((nx, ny, dx, dy))
            }
        }
    })
    .take(4)
    .map(|(x, y, _, _)| c[y][x])
    .collect::<String>();
    println!("{}", ans);
}
