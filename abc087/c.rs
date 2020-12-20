#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_cols<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn main() {
    let n = read::<usize>();
    let upper_cols = read_cols::<i32>();
    let lower_cols = read_cols::<i32>();

    assert_eq!(n, upper_cols.len());
    assert_eq!(n, lower_cols.len());

    let lower_sum: i32 = lower_cols.iter().sum::<i32>();

    let m = upper_cols
        .iter()
        .zip((0..1).chain(lower_cols.into_iter()))
        .map(|(u, l)| u - l)
        .scan(lower_sum, |s, diff| {
            *s += diff;
            Some(*s)
        })
        .max()
        .unwrap();

    println!("{}", m);
}
