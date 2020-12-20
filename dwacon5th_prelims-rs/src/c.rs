#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_macros)]
macro_rules! read_cols {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn main() {
    let _n: usize = read();
    let s: Vec<_> = read::<String>().chars().collect();
    let _q: usize = read();

    let ks = read_vec::<usize>();

    for k in ks {
        let ans = s
            .iter()
            .copied()
            .zip(std::iter::repeat('X').take(k).chain(s.iter().copied()))
            .fold(
                (0usize, 0usize, 0usize, 0usize),
                |(nd, nm, ndm, ans), (c, d)| {
                    let (nd, nm, ndm) = match d {
                        'D' => (nd - 1, nm, ndm - nm),
                        'M' => (nd, nm - 1, ndm),
                        _ => (nd, nm, ndm),
                    };

                    match c {
                        'D' => (nd + 1, nm, ndm, ans),
                        'M' => (nd, nm + 1, ndm + nd, ans),
                        'C' => (nd, nm, ndm, ans + ndm),
                        _ => (nd, nm, ndm, ans),
                    }
                },
            )
            .3;
        println!("{}", ans);
    }
}
