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
    let (n, mut x, mut d) = read_cols!(usize, i64, i64);

    if d < 0 {
        x = -x;
        d = -d;
    }

    if x == 0 && d == 0 {
        println!("1");
        return;
    }

    if d == 0 {
        println!("{}", n + 1);
        return;
    }

    let g = num::integer::gcd(x, d);
    let xx = x / g;
    let dd = d / g;

    let ans = (0..=n)
        .fold(vec![vec![]; min(dd as usize, n + 1)], |mut state, i| {
            let lower = (i as i64) * (i as i64 - 1) / 2;
            let upper = (n * i - i * (i + 1) / 2 + 1) as i64;

            assert!(lower <= upper);

            let offset = (i as i64 / dd) * xx;
            state[i % (dd as usize)].push((lower + offset, upper + offset));
            state
        })
        .into_iter()
        .map(|mut v| {
            v.sort();

            v.into_iter()
                .fold((-(1 << 60), 0), |(prev, acc), (lower, upper)| {
                    if prev <= lower {
                        (upper, acc + (upper - lower))
                    } else if prev < upper {
                        (upper, acc + (upper - prev))
                    } else {
                        (prev, acc)
                    }
                })
                .1
        })
        .sum::<i64>();

    println!("{}", ans);
}
