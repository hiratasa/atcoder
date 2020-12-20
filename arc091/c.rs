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
#[allow(unused_imports)]
use std::collections::VecDeque;
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
    // 内部→9回裏返される→裏
    // 辺→6回裏返される→表
    // 角→4回裏返される→表
    // 裏の枚数＝内部の枚数＝(N-2)*(M-2)
    let (n, m) = read_cols!(u64, u64);

    let ans = if n == 1 {
        if m == 1 {
            1
        } else {
            // 端以外→3回→裏
            // 端→2回→表
            // 裏の枚数=m-2
            m - 2
        }
    } else {
        if m == 1 {
            n - 2
        } else {
            (n - 2) * (m - 2)
        }
    };

    println!("{}", ans);
}