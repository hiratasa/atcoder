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
    let n: usize = read();

    let mut ab = (0..n)
        .map(|_| read_cols!(usize, usize))
        .enumerate()
        .collect::<Vec<_>>();

    ab.sort_by_key(|&(i, (a, b))| (a, b, i));

    let ans0 = {
        let mut ans0 = ab
            .iter()
            .rev()
            .copied()
            .fold(vec![usize::MAX], |mut ans0, (_, (a, b))| {
                if b <= *ans0.last().unwrap() {
                    ans0.push(a);
                }

                ans0
            });

        ans0.reverse();

        ans0
    };

    ab.sort_by_key(|&(i, (a, b))| (b, a, i));

    let ans = ab
        .iter()
        .copied()
        .fold(
            (vec![(0usize, 0usize); 0], 0usize, 0usize),
            |(mut ans, mut prev, mut idx), (i, (a, b))| {
                if ans0[idx + 1] < b {
                    assert!(ans.len() == idx + 1);

                    prev = ans[idx].1;
                    idx += 1;
                }

                if prev <= a {
                    if idx < ans.len() {
                        if i < ans[idx].0 {
                            ans[idx] = (i, b);
                        }
                    } else {
                        ans.push((i, b));
                    }
                }

                (ans, prev, idx)
            },
        )
        .0;

    println!("{}", ans.len());

    let mut delim = "";
    for a in ans {
        print!("{}{}", delim, a.0 + 1);
        delim = " ";
    }
    println!("");
}
