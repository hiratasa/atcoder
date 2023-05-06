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
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

const M: i64 = 998244353;

fn main() {
    let (n, k) = read_cols!(usize, usize);

    let lr = (0..k).map(|_| read_cols!(usize, usize)).collect::<Vec<_>>();

    let mut d = vec![0; n];

    let mut c = 0;
    d[0] = 1;
    d[1] = M - 1;
    for i in 0..n {
        c += d[i];
        c %= M;
        for (l, r) in &lr {
            if i + l >= n {
                continue;
            }

            d[i + l] += c;
            d[i + l] %= M;
            if i + r + 1 < n {
                d[i + r + 1] += M - c;
                d[i + r + 1] %= M;
            }
        }
    }

    println!("{}", c);
}
