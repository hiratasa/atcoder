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
    let (n, q) = read_cols!(usize, usize);

    let a = read_vec::<usize>()
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();

    let c = std::iter::once(0)
        .chain(a.iter().map(|(_, aa)| aa).scan(0usize, |acc, aa| {
            *acc += aa;
            Some(*acc)
        }))
        .collect::<Vec<_>>();
    let d = std::iter::repeat(0usize)
        .take(2)
        .chain(
            a.iter()
                .map(|(_, aa)| aa)
                .scan((0usize, 0usize), |(acc0, acc1), aa| {
                    swap(acc0, acc1);
                    *acc1 += aa;
                    Some(*acc1)
                }),
        )
        .collect::<Vec<_>>();

    let x = (0..q).map(|_| read()).collect::<Vec<usize>>();

    x.iter()
        .map(|&xx| {
            let i = a
                .binary_search_by(|&(i, aa)| {
                    if aa < xx {
                        Ordering::Less
                    } else if xx < aa - xx {
                        if 2 * i + 1 < n {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else if 2 * i + 1 < n {
                        Ordering::Less
                    } else if a[n + 1 - 2 * (n - i)].1 < xx - (aa - xx) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap_err();

            assert!(i < n || a[n - 1].1 < xx);

            let j = if i == n {
                n + 1
            } else if xx < a[i].1 - xx {
                0
            } else {
                n + 1 - 2 * (n - i)
            };

            c[n] - c[i] + d[j]
        })
        .for_each(|xx| println!("{}", xx));
}
