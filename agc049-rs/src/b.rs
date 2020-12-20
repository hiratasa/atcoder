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

    let s = read::<String>()
        .chars()
        .map(|c| if c == '1' { 1 } else { 0 })
        .collect::<Vec<usize>>();
    let t = read::<String>()
        .chars()
        .map(|c| if c == '1' { 1 } else { 0 })
        .collect::<Vec<_>>();

    let (prev, unmatched, ans) = s.iter().copied().zip(t.iter().copied()).enumerate().fold(
        (None, VecDeque::new(), 0),
        |(prev, mut unmatched, ans): (Option<usize>, VecDeque<usize>, usize), (i, (ss, tt))| {
            if tt > 0 {
                unmatched.push_back(i);
            }
            if ss == 0 {
                (prev, unmatched, ans)
            } else if let Some(prev) = prev {
                (None, unmatched, ans + (i - prev))
            } else if let Some(&j) = unmatched.front() {
                unmatched.pop_front();
                (None, unmatched, ans + (i - j))
            } else {
                (Some(i), unmatched, ans)
            }
        },
    );

    if let Some(_) = prev {
        println!("-1");
    } else if !unmatched.is_empty() {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
