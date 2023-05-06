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

fn main() {
    let (_n, x) = read_cols!(usize, usize);

    let a = read_vec::<usize>();

    let b = {
        let mut b = a
            .iter()
            .rev()
            .scan(x, |xx, &aa| {
                let bb = *xx / aa;
                *xx %= aa;
                Some(bb)
            })
            .collect::<Vec<_>>();
        b.reverse();
        b
    };

    let c = a
        .windows(2)
        .map(|s| s[1] / s[0])
        .chain(std::iter::once(std::usize::MAX))
        .collect::<Vec<_>>();

    let ans = b
        .iter()
        .zip(c.iter())
        .fold((1usize, 0usize), |(no_carry, carry), (&bb, &cc)| {
            if bb == 0 {
                (no_carry + carry, carry)
            } else if bb == cc - 1 {
                (no_carry, no_carry + carry)
            } else {
                (no_carry + carry, no_carry + carry)
            }
        })
        .0;
    println!("{}", ans);
}
