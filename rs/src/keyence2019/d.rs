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
    let (n, m) = read_cols!(usize, usize);

    let a = {
        let mut a = read_vec::<usize>();
        a.sort_by_key(|aa| std::cmp::Reverse(*aa));
        a.dedup();
        a.push(0); // sentinel
        a
    };

    let b = {
        let mut a = read_vec::<usize>();
        a.sort_by_key(|aa| std::cmp::Reverse(*aa));
        a.dedup();
        a.push(0); // sentinel
        a
    };

    if *a.first().unwrap() != n * m || a.len() != n + 1 {
        println!("0");
        return;
    }

    if *b.first().unwrap() != n * m || b.len() != m + 1 {
        println!("0");
        return;
    }

    let ans = (1..=n * m)
        .rev()
        .scan((0, 0, 0), |(nn, mm, c), i| {
            if a[*nn] == i && b[*mm] == i {
                *nn += 1;
                *mm += 1;
                *c += *nn + *mm - 2;
                Some(1)
            } else if a[*nn] == i {
                *nn += 1;
                *c += *mm - 1;
                Some(*mm)
            } else if b[*mm] == i {
                *mm += 1;
                *c += *nn - 1;
                Some(*nn)
            } else {
                if *c == 0 {
                    Some(0)
                } else {
                    *c -= 1;
                    Some(*c + 1)
                }
            }
        })
        .fold(1, |prod, a| prod * a % 1000000007);
    println!("{}", ans);
}
