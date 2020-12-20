#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::iter;
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
    let (a, b, q) = read_cols!(usize, usize, usize);

    let s = {
        let mut s = vec![];
        s.push(-100000000000);
        for _ in 0..a {
            s.push(read::<i64>());
        }
        s.push(100000000000);
        s.sort();
        s
    };
    let t = {
        let mut t = vec![];
        t.push(-100000000000);
        for _ in 0..b {
            t.push(read::<i64>());
        }
        t.push(100000000000);
        t.sort();
        t
    };

    for _ in 0..q {
        let x = read::<i64>();
        let is = s.binary_search(&x);
        let it = t.binary_search(&x);

        let ls = s[is.unwrap_or_else(|i| i - 1)];
        let lt = t[it.unwrap_or_else(|i| i - 1)];

        let rs = s[is.unwrap_or_else(|i| i)];
        let rt = t[it.unwrap_or_else(|i| i)];

        let p1 = x - min(ls, lt);
        let p2 = 2 * (x - ls) + (rt - x);
        let p3 = 2 * (x - lt) + (rs - x);
        let p4 = 2 * (rt - x) + (x - ls);
        let p5 = 2 * (rs - x) + (x - lt);
        let p6 = max(rs, rt) - x;

        let m = min(p1, min(p2, min(p3, min(p4, min(p5, p6)))));
        println!("{}", m);
    }
}