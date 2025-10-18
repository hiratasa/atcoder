#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::once;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

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
    let s = read_str();

    let ans = if s.len() == 26 {
        match s
            .iter()
            .copied()
            .enumerate()
            .rev()
            .scan('0', |m, (i, c)| {
                *m = std::cmp::max(*m, c);
                Some((i, c, *m == c))
            })
            .find(|(_, _, is_max)| !is_max)
        {
            None => String::from("-1"),
            Some((idx, _, _)) => {
                let (p1, p2) = s.split_at(idx);
                let c = p2
                    .iter()
                    .copied()
                    .skip(1)
                    .filter(|&c| c > p2[0])
                    .min()
                    .unwrap();
                chain(p1.iter().copied(), once(c)).collect::<String>()
            }
        }
    } else {
        let used = s.iter().copied().fold(BTreeSet::new(), |mut used, c| {
            used.insert(c);
            used
        });

        let first_unused = ('a' as u8..='z' as u8)
            .find(|&c| !used.contains(&(c as char)))
            .unwrap() as char;

        chain(s, once(first_unused)).collect::<String>()
    };

    println!("{}", ans);
}
