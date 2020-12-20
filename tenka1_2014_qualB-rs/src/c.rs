#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

fn solve(c: &Vec<Vec<bool>>, d0: &Vec<bool>) -> Vec<Vec<bool>> {
    let n = c[0].len();

    c.iter().take(n - 1).fold(vec![d0.clone()], |mut d, cc| {
        d.push(
            (0..n)
                .map(|i| {
                    cc[i]
                        ^ (if d.len() >= 2 {
                            d[d.len() - 2][i]
                        } else {
                            false
                        })
                        ^ (if i > 0 { d[d.len() - 1][i - 1] } else { false })
                        ^ (if i < n - 1 {
                            d[d.len() - 1][i + 1]
                        } else {
                            false
                        })
                })
                .collect::<Vec<_>>(),
        );
        d
    })
}

fn main() {
    let n: usize = read();
    let c = (0..n)
        .map(|_| {
            read::<String>()
                .chars()
                .map(|cc| cc == '#')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if n == 1 {
        println!(".");
        return;
    }

    let ans = solve(&c, &vec![false; n]);

    for a in ans {
        println!(
            "{}",
            a.into_iter()
                .map(|aa| if aa { '#' } else { '.' })
                .collect::<String>()
        );
    }
}
