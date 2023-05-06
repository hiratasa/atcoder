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
    let _n: usize = read();
    let mut a = read_vec::<usize>();

    a.sort();

    let mut m = a.iter().copied().fold(BTreeMap::new(), |mut m, aa| {
        *m.entry(aa).or_insert(0) += 1;
        m
    });

    let mut ans: usize = 0;
    for &aa in a.iter().rev() {
        if *m.get(&aa).unwrap() == 0 {
            continue;
        }

        let b = (aa + 1).next_power_of_two() - aa;

        if b == aa {
            if m.get(&b).copied().unwrap_or(0) >= 2 {
                ans += 1;
                m.insert(b, m.get(&b).copied().unwrap() - 2);
            }
        } else {
            if m.get(&b).copied().unwrap_or(0) >= 1 {
                ans += 1;
                m.insert(aa, m.get(&aa).copied().unwrap() - 1);
                m.insert(b, m.get(&b).copied().unwrap() - 1);
            }
        }
    }

    println!("{}", ans);
}
