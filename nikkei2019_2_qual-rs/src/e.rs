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
    let (n, k) = read_cols!(usize, usize);

    if 2 * k > n + 1 {
        println!("-1");
        return;
    }

    if n % 2 == 0 {
        // k <= n/2
        // => (a, b) = (k, k + n * 3/2 - 1), (k + 2, k + n * 3/2 - 2), ..., (k + n - 2, k + n),
        //             (k + 1, k + 2 * n - 1), (k + 3, k + 2 * n - 2), ..., (k + n - 1, k + n * 3/2)
        for i in 0..n / 2 {
            println!("{} {} {}", k + 2 * i, k + n * 3 / 2 - 1 - i, k + 2 * n + i);
        }
        for i in 0..n / 2 {
            println!(
                "{} {} {}",
                k + 1 + 2 * i,
                k + 2 * n - 1 - i,
                k + 5 * n / 2 + i,
            );
        }
    } else {
        // k <= (n+1)/2
        // => (a, b) = (k, k + (3 * n - 1) / 2), (k + 2, k + (3 * n - 3) / 2), ..., (k + n - 1, k + n),
        //             (k + 1, k + 2 * n - 1), (k + 3, k + 2 * n - 2), ..., (k + n - 2, k + (3n + 1) / 2)
        for i in 0..(n + 1) / 2 {
            println!(
                "{} {} {}",
                k + 2 * i,
                k + (3 * n - 1) / 2 - i,
                k + 2 * n + i,
            );
        }
        for i in 0..(n - 1) / 2 {
            println!(
                "{} {} {}",
                k + 1 + 2 * i,
                k + 2 * n - 1 - i,
                k + (5 * n + 1) / 2 + i,
            );
        }
    }
}
