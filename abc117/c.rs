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
    let (n, m) = read_cols!(usize, usize);
    let nums = {
        let mut nums = read_vec::<i64>();
        nums.sort();
        nums
    };
    assert_eq!(m, nums.len());

    if n >= m {
        println!("0");
        return;
    }

    let mut dist: Vec<_> = nums.iter().skip(1).zip(nums.iter())
                    .map(|(x, y)| x - y)
                    .collect::<Vec<_>>();
    dist.sort();
    let ans: i64 = dist.iter().take(m - n).sum();
    
    println!("{}", ans);
}