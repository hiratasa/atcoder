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

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

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

const M: i64 = 1_000_000_007;
const INV2: i64 = 500_000_004;

fn main() {
    let l: i64 = read();

    // b = a + 1, a + 1 < c < 2 * a + 1
    // a + 1 < l - (2 * a + 1)  ---->  a < (l - 2) / 3
    // l - (2 * a + 1) < 2 * a + 1  --->  a > (l - 2) / 4
    // sum[l=6 to L] (l / 3 - (l + 2) / 4)

    // b = a + 1, c < a, a + 1 < a + c
    // l - (2 * a + 1) < a  --->  a > (l - 1) / 3
    // 1 < l - (2 * a + 1)  --->  a < l / 2 - 1
    // sum[l=6 to L] ((l + 1) / 2 - 1 - (l + 2) / 3)

    // b = a + 1, c = a + 2
    // => lが9以上の3の倍数のとき1引く

    if l == 5 {
        println!("0");
        return;
    }

    let ans = (3 + l / 2) % M * ((l / 2 - 2) % M) % M * INV2 % M
        + (4 + (l + 1) / 2) % M * (((l + 1) / 2 - 3) % M) % M * INV2 % M
        + M
        - (l - 5) % M
        + M
        - (l / 3 - 2) % M
        + M
        - ((l - 1) / 3 - 1) % M
        + M
        - ((l - 2) / 3 - 1) % M
        + M
        - (2 + l / 4) % M * ((l / 4 - 1) % M) % M * INV2 % M
        + M
        - (2 + (l - 1) / 4) % M * (((l - 1) / 4 - 1) % M) % M * INV2 % M
        + M
        - (2 + (l + 2) / 4) % M * (((l + 2) / 4 - 1) % M) % M * INV2 % M
        + M
        - (2 + (l + 1) / 4) % M * (((l + 1) / 4 - 1) % M) % M * INV2 % M;
    let ans = ans % M;

    println!("{}", ans);
}
