#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    let (n, k) = read_cols!(usize, usize);
    let nums = read_vec::<usize>();

    let mask = (0..40)
        .filter(|i| n > 2 * nums.iter().filter(|&m| ((m >> i) & 1usize) > 0usize).count())
        .fold(0usize, |acc, i| acc + (1usize << i));
    
    let ans:usize = (0..40)
        .map(|i| (k & mask & !((1usize << (i + 1)) - 1)) + (mask & ((1usize << i) - 1)))
        .filter(|&x| x <= k)
        .map(|x| nums.iter().map(|a| a ^ x).sum())
        .max()
        .unwrap();
    
    println!("{}", ans);
}