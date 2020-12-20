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

fn main() {
    let n: usize = read();
    let xy = { (0..n).map(|_| read_cols!(i64, i64)).collect::<Vec<_>>() };

    let ans = max(
        {
            let mi = xy.iter().map(|(x, y)| x - y).min().unwrap();
            let ma = xy.iter().map(|(x, y)| x - y).max().unwrap();

            (mi - ma).abs()
        },
        {
            let mi = xy.iter().map(|(x, y)| x + y).min().unwrap();
            let ma = xy.iter().map(|(x, y)| x + y).max().unwrap();

            (mi - ma).abs()
        },
    );

    println!("{}", ans);
}
