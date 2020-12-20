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
    let (h, w) = read_tuple!(usize, usize);

    let ab = read_vec(h, || read_tuple!(usize, usize));
    let ab = ab
        .iter()
        .copied()
        .map(|(a, b)| (a - 1, b - 1))
        .collect_vec();

    let m = (0..w).map(|i| (i, 0)).collect::<BTreeMap<_, _>>();
    let q = (0..w).map(|i| Reverse((0, i))).collect::<BinaryHeap<_>>();

    let get_value = |m: &BTreeMap<usize, usize>, x: usize| {
        m.range(..=x).next_back().map(|(&x0, &c)| c + (x - x0))
    };

    ab.iter()
        .copied()
        .enumerate()
        .scan((m, q), |(m, q), (y, (a, b))| {
            if let Some(c) = get_value(m, b + 1) {
                m.insert(b + 1, c);
            }

            // This cannot be done by for-loop (because range() borrows m).
            while let Some((&x, _)) = m.range(a..=b).next() {
                m.remove(&x);
            }

            if a > 0 {
                if let Some(c) = get_value(m, a - 1) {
                    q.push(Reverse((c, a - 1)));
                }
            }

            if b < w - 1 {
                if let Some(c) = get_value(m, b + 1) {
                    q.push(Reverse((c, b + 1)));
                }
            }

            while let Some(&Reverse((c, x))) = q.peek() {
                if matches!(get_value(m, x), Some(c2) if c == c2) {
                    return Some(Some(y + 1 + c));
                }

                q.pop();
            }

            Some(None)
        })
        .for_each(|c| {
            if let Some(c) = c {
                println!("{}", c);
            } else {
                println!("-1");
            }
        });
}
