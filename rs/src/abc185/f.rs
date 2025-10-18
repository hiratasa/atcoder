#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

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
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

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
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

struct BIT {
    len: usize,
    values: Vec<usize>,
}

#[allow(dead_code)]
impl BIT {
    fn new(len: usize) -> BIT {
        BIT {
            len,
            values: vec![0usize; len],
        }
    }

    fn xor(&self, i: usize) -> usize {
        let mut s = 0;
        let mut idx = i as i64;

        // values[1] ~ values[i] の和
        // (bは1-indexedなのでこれでOK)
        while idx > 0 {
            s = s ^ self.values[(idx - 1) as usize];
            idx -= idx & -idx;
        }

        return s;
    }

    fn xor_between(&self, i: usize, j: usize) -> usize {
        self.xor(j) ^ self.xor(i)
    }

    fn add(&mut self, i: usize, a: usize) {
        // 1-indexedに直す
        let mut idx = i as i64 + 1;

        while idx as usize <= self.len {
            self.values[(idx - 1) as usize] = self.values[(idx - 1) as usize] ^ a;
            idx += idx & -idx;
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let (n, q) = read_tuple!(usize, usize);

    let a = read_row::<usize>();

    let query = read_vec(q, || read_tuple!(usize, usize, usize));

    let mut bit = BIT::new(n);
    for (i, &aa) in a.iter().enumerate() {
        bit.add(i, aa);
    }

    for &(t, x, y) in &query {
        let x = x - 1;
        if t == 1 {
            bit.add(x, y);
        } else {
            println!("{}", bit.xor_between(x, y));
        }
    }
}
