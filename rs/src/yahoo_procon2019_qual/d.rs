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
    let l = read::<usize>();

    let a = (0..l).map(|_| read::<usize>()).collect::<Vec<_>>();

    let ans = a.iter().fold(
        (0usize, 0usize, 0usize, 0usize, 0usize),
        |(dp0, dp1, dp2, dp3, dp4), aa| {
            let mdp1 = min(dp0, dp1);
            let mdp2 = min(mdp1, dp2);
            let mdp3 = min(mdp2, dp3);
            let mdp4 = min(mdp3, dp4);
            match aa {
                0 => (dp0, mdp1 + 2, mdp2 + 1, mdp3 + 2, mdp4),
                _ if aa % 2 == 0 => (dp0 + aa, mdp1, mdp2 + 1, mdp3, mdp4 + aa),
                _ => (dp0 + aa, mdp1 + 1, mdp2, mdp3 + 1, mdp4 + aa),
            }
        },
    );
    eprintln!("{:?}", ans);
    let ans = [ans.0, ans.1, ans.2, ans.3, ans.4]
        .iter()
        .copied()
        .min()
        .unwrap();

    println!("{}", ans);
}
