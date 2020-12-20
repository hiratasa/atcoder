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
    let (n, m) = read_cols!(usize, usize);

    let ab = {
        let mut ab = (0..n).map(|_| read_cols!(usize, usize)).collect::<Vec<_>>();
        ab.sort_by_key(|&(a, b)| (b, a));
        ab
    };

    let cd = {
        let mut cd = (0..m).map(|_| read_cols!(usize, usize)).collect::<Vec<_>>();
        cd.sort();
        cd
    };

    let ans = ab
        .iter()
        .copied()
        .rev()
        .fold(
            (0usize, BTreeMap::new(), cd.iter().copied().rev().peekable()),
            |(ans, mut map, mut it), (a, b)| {
                while let Some(&(c, d)) = it.peek() {
                    if c < b {
                        break;
                    }

                    it.next();
                    *map.entry(d).or_insert(0) += 1;
                }

                if let Some((&d, &x)) = map.range(0..=a).next_back() {
                    if x == 1 {
                        map.remove(&d);
                    } else {
                        map.insert(d, x - 1);
                    }

                    (ans + 1, map, it)
                } else {
                    (ans, map, it)
                }
            },
        )
        .0;

    println!("{}", ans);
}
