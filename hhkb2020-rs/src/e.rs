#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
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
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

const M: usize = 1000000007;

fn main() {
    let (h, w) = read_cols!(usize, usize);

    let s = {
        let mut v: Vec<Vec<_>> = vec![];
        for _ in 0..h {
            let line: String = read();
            v.push(line.chars().map(|c| c == '.').collect());
        }
        v
    };

    let k: usize = s
        .iter()
        .map(|row| row.iter().map(|&b| if b { 1 } else { 0 }).sum::<usize>())
        .sum();

    let up: Vec<_> = (0..h)
        .scan(vec![], |state: &mut Vec<usize>, i| {
            *state = if i > 0 {
                state
                    .iter()
                    .enumerate()
                    .map(|(j, v)| if s[i][j] { v + 1 } else { 0 })
                    .collect()
            } else {
                s[0].iter().map(|&b| if b { 1 } else { 0 }).collect()
            };
            Some(state.clone())
        })
        .collect();

    let down: Vec<_> = (0..h)
        .rev()
        .scan(vec![], |state: &mut Vec<usize>, i| {
            *state = if i < h - 1 {
                state
                    .iter()
                    .enumerate()
                    .map(|(j, v)| if s[i][j] { v + 1 } else { 0 })
                    .collect()
            } else {
                s[i].iter().map(|&b| if b { 1 } else { 0 }).collect()
            };
            Some(state.clone())
        })
        .collect::<Vec<_>>();

    let left: Vec<_> = (0..w)
        .scan(vec![], |state: &mut Vec<usize>, i| {
            *state = if i > 0 {
                state
                    .iter()
                    .enumerate()
                    .map(|(j, v)| if s[j][i] { v + 1 } else { 0 })
                    .collect()
            } else {
                (0..h).map(|j| if s[j][i] { 1 } else { 0 }).collect()
            };
            Some(state.clone())
        })
        .collect();

    let right: Vec<_> = (0..w)
        .rev()
        .scan(vec![], |state: &mut Vec<usize>, i| {
            *state = if i < w - 1 {
                state
                    .iter()
                    .enumerate()
                    .map(|(j, v)| if s[j][i] { v + 1 } else { 0 })
                    .collect()
            } else {
                (0..h).map(|j| if s[j][i] { 1 } else { 0 }).collect()
            };
            Some(state.clone())
        })
        .collect();

    let pow2: Vec<_> = (0..=h * w)
        .scan(1, |p, i| {
            if i > 0 {
                *p *= 2;
                *p %= M;
            }
            Some(*p)
        })
        .collect();

    let ans = s
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &b)| {
                    if b {
                        let r =
                            up[i][j] + down[h - 1 - i][j] + left[j][i] + right[w - 1 - j][i] - 3;
                        (pow2[k] + M - pow2[k - r]) % M
                    } else {
                        0
                    }
                })
                .fold(0, |acc, m| (acc + m) % M)
        })
        .fold(0, |acc, m| (acc + m) % M);
    println!("{}", ans);
}
