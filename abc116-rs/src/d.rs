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
    let (n, k) = read_cols!(usize, usize);

    let td = (0..n).map(|_| read_cols!(usize, usize)).collect::<Vec<_>>();

    let (mut tt, r): (Vec<_>, Vec<_>) = td
        .iter()
        .copied()
        .fold(vec![vec![]; n + 1], |mut tt, (t, d)| {
            tt[t].push(d);
            tt
        })
        .into_iter()
        .filter(|ttt| !ttt.is_empty())
        .map(|mut ttt| {
            ttt.sort();
            let r = ttt.split_off(ttt.len() - 1);
            (r[0], ttt)
        })
        .unzip();

    tt.sort_by_key(|&a| std::cmp::Reverse(a));

    let mut r = r.into_iter().flatten().collect::<Vec<_>>();
    r.sort_by_key(|&a| std::cmp::Reverse(a));

    let stt = std::iter::once(0)
        .chain(tt.iter().copied())
        .scan(0, |acc, a| {
            *acc += a;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let sr = std::iter::once(0)
        .chain(r.iter().copied())
        .scan(0, |acc, a| {
            *acc += a;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let ans = (1..=k)
        .take(tt.len())
        .skip_while(|i| k > r.len() + i)
        // .inspect(|i| println!("{},", i))
        .map(|i| i * i + stt[i] + sr[k - i])
        // .inspect(|v| println!("{}", v))
        .max()
        .unwrap();
    println!("{}", ans);
}
