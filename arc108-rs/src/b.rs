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

fn main() {
    let n: usize = read::<usize>();

    let s = read::<String>();

    let ans = s
        .chars()
        .fold((n, vec![]), |(ans, mut v): (usize, Vec<usize>), c| {
            if c == 'f' {
                v.push(1);
                (ans, v)
            } else if let Some(&a) = v.last() {
                match a {
                    1 => {
                        if c == 'o' {
                            *v.last_mut().unwrap() += 1;
                            (ans, v)
                        } else {
                            v.clear();
                            (ans, v)
                        }
                    }
                    2 => {
                        if c == 'x' {
                            v.pop();
                            return (ans - 3, v);
                        } else {
                            v.clear();
                            (ans, v)
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                (ans, v)
            }
        })
        .0;

    println!("{}", ans);
}
