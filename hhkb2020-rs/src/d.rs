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
    let t: usize = read();

    for _ in 0..t {
        let (n, a, b) = read_cols!(usize, usize, usize);

        const M: usize = 1000000007;

        if a + b > n {
            println!("0");
            continue;
        }

        // sum [min(x + a, n - b + 1)- max(x - b + 1, 0)] from x = 0 to n - a
        // = sum (x + a) from x = 0 to (n - a - b)
        //  +sum (n - b + 1) from x = (n - a - b + 1) to n - a
        //  -sum (x - b + 1) from x = b to n - a
        // = (n + a - b) * (n - a - b + 1) / 2 + b * (n - b + 1) - (n - a - b + 2) * (n - a - b + 1) / 2
        // = (a - 1) * (n - a - b + 1) + b * (n - b + 1)
        let c = ((a - 1) * (n - a - b + 1) + b * (n - b + 1)) % M;
        let ans = ((n - a + 1) * (n - a + 1) % M * (n - b + 1) % M * (n - b + 1) % M
            + (M - c * c % M))
            % M;
        println!("{}", ans);
    }
}
