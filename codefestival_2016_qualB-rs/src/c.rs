#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
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

use std::collections::binary_heap::*;

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
    let (w, h) = read_cols!(usize, usize);

    let p = (0..w).map(|_| read()).collect::<Vec<i64>>();
    let q = (0..h).map(|_| read()).collect::<Vec<i64>>();

    let mut heap = BinaryHeap::new();

    for pp in p {
        heap.push((-pp, false));
    }
    for qq in q {
        heap.push((-qq, true));
    }

    let mut ww = (w + 1) as i64;
    let mut hh = (h + 1) as i64;
    let mut ans = 0;
    while let Some((t, dir)) = heap.pop() {
        let cost = -t;

        if dir {
            ans += cost * ww;
            hh -= 1;
        } else {
            ans += cost * hh;
            ww -= 1;
        }
    }

    println!("{}", ans);
}
