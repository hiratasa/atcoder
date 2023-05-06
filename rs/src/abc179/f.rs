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
    let (n, q) = read_cols!(i64, usize);

    let mut mx = vec![(n, n)];
    let mut my = vec![(n, n)];

    let white: i64 = (0..q)
        .map(|_| {
            let (t, x) = read_cols!(i64, i64);

            // eprintln!("{:?}", my);
            // eprintln!("{:?}", mx);

            if t == 1 {
                let r = my
                    .binary_search_by_key(&Reverse((x, 0)), |e| Reverse(*e))
                    .unwrap_err();
                let ty = my[r - 1].1;

                if (ty, x) < *mx.last().unwrap() {
                    mx.push((ty, x));
                }

                ty - 2
            } else {
                let r = mx
                    .binary_search_by_key(&Reverse((x, 0)), |e| Reverse(*e))
                    .unwrap_err();
                let tx = mx[r - 1].1;

                if (tx, x) < *my.last().unwrap() {
                    my.push((tx, x));
                }

                tx - 2
            }
        })
        .sum();

    println!("{}", (n - 2) * (n - 2) - white);
}
