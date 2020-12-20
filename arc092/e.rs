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

fn max_each_start(a: &Vec<i64>, start: usize) -> (usize, i64) {
    a.iter()
        .map(|e| *e)
        .skip(start)
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, k)| k)
        .scan(0, |s, k| {
            *s += k;
            Some(*s)
        })
        .enumerate()
        .max_by(|&(_, k), &(_, l)| k.cmp(&l))
        .map(|(i, k)| (i + 1, k))
        .unwrap()
}

fn solve(a: &Vec<i64>) -> (i64, usize, usize) {
    let mut ret = (i64::min_value(), 0, 0);
    let n = a.len();

    // even
    let mut prev = -1;
    for i in 0..(n + 1) / 2 {
        let index = 2 * i;
        if prev < 0 {
            let v = max_each_start(a, index);
            if v.1 > ret.0 {
                ret.0 = v.1;
                ret.1 = index;
                ret.2 = v.0;
            }
        }
        prev = a[index];
    }

    // odd
    let mut prev = -1;
    for i in 0..n / 2 {
        let index = 2 * i + 1;
        if prev < 0 {
            let v = max_each_start(a, index);
            if v.1 > ret.0 {
                ret.0 = v.1;
                ret.1 = index;
                ret.2 = v.0;
            }
        }
        prev = a[index];
    }

    ret
}

fn main() {
    let n = read::<usize>();
    let a = read_vec::<i64>();
    assert_eq!(a.len(), n);

    let (m, start, len) = solve(&a);

    println!("{}", m);
    println!("{}", n - len);
    for _ in 0..len-1 {
        // startの右を選ぶ
        // 1始まりに注意
        println!("{}", start + 2);
    }
    // 残り n - 2*(len-1) 個
    for _ in 0..start {
        println!("{}", 1);
    }
    // 残り n - 2*(len-1) - start 個
    for i in 0..n-2*(len - 1) - start - 1 {
        println!("{}", n - 2 * (len - 1) - start - i);
    }
}
