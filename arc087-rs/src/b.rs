#[allow(unused_imports)]
use bitset_fixed::BitSet;
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
macro_rules! read_tuple {
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
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

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

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

fn main() {
    let s = read_str();

    let (x, y) = read_tuple!(i64, i64);

    let t = s.iter().fold(([vec![0usize], vec![0usize]], 0usize), |(mut t, idx), c| {
        match c {
            'F' => {
                *t[idx].last_mut().unwrap() += 1;
                (t, idx)
            },
            'T' => {
                let idx = (idx + 1) % 2;
                t[idx].push(0);
                (t, idx)
            }
            _ => unreachable!()
        }
    }).0;

    const OFFSET: usize = 8000;

    let bs_x = {
        let mut bs = BitSet::new(2 * OFFSET + 1);
        bs.set(t[0][0] + OFFSET, true);
        bs
    };

    let bs_y = {
        let mut bs = BitSet::new(2 * OFFSET + 1);
        bs.set(0 + OFFSET, true);
        bs
    };

    let ok_x = t[0].iter().skip(1).copied().fold(bs_x, |bs_x, m| {
        (&bs_x << m) | &(&bs_x >> m)
    })[(x + OFFSET as i64) as usize];

    let ok_y = t[1].iter().copied().fold(bs_y, |bs_y, m| {
        (&bs_y << m) | &(&bs_y >> m)
    })[(y + OFFSET as i64) as usize];

    if ok_x && ok_y {
        println!("Yes");
    } else {
        println!("No");
    }
}
