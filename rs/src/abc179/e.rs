#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::iter::*;
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

fn main() {
    let (n, x, m) = read_cols!(i64, usize, usize);

    let a0 = (0..m).map(|i| (i, i * i % m)).collect::<Vec<_>>();

    let a = once((1, a0.clone()))
        .chain((1..=34).scan((1, a0), |aa, _| {
            aa.0 = 2 * aa.0;
            aa.1 = (0..m)
                .map(|i| {
                    let j = aa.1[i].1;
                    (aa.1[i].0 + aa.1[j].0, aa.1[j].1)
                })
                .collect();

            Some(aa.clone())
        }))
        .collect::<Vec<_>>();

    let ans = a
        .iter()
        .rev()
        .fold((n, x, 0), |(nn, xx, s), (p, b)| {
            let p = *p;
            if nn < p {
                (nn, xx, s)
            } else {
                (nn - p, b[xx].1, s + b[xx].0)
            }
        })
        .2;

    println!("{}", ans);
}
