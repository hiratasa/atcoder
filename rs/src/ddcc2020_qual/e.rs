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
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
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
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    }
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let mut c = $c;
        c.push($x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
    }};
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

trait IterCopyExt<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

impl<'a, T, I> IterCopyExt<'a, T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
}

fn query<I: IntoIterator<Item = usize>>(a: I) -> bool {
    println!("? {}", a.into_iter().join(" "));
    let c = read::<String>();

    if c == "-1" {
        std::process::exit(1);
    }

    c == "Red"
}

fn main() {
    let n: usize = read();

    let c = query(1..=n);

    let mut begin = 1;
    let mut end = n + 1;
    while begin + 1 < end {
        // [1, begin) に+nしても色が変わらない
        // [1, end) に+nすると色が変わる
        let mid = (begin + end) / 2;

        let d = query((1..mid).map(|i| i + n).chain(mid..=n));
        if d == c {
            begin = mid;
        } else {
            end = mid;
        }
    }
    assert!(begin + 1 == end);

    let c_idx = begin;

    // c_idxの色がc
    // c_idx+nの色が!c
    // [1+n, c_idx+n) + [c_idx, n] で c <-> !c するとmajorityが変わる

    let a0 = (1 + n..c_idx + n).chain(c_idx..=n).collect_vec();
    let a1 = (1..c_idx).chain(c_idx + n..=2 * n).collect_vec();

    let ans = a0
        .citer()
        .map(|i| {
            if i == c_idx {
                (i, c)
            } else {
                let d = query(a0.citer().map(|j| if j == i { c_idx + n } else { j }));
                if d == c { (i, !c) } else { (i, c) }
            }
        })
        .chain(a1.citer().map(|i| {
            if i == c_idx + n {
                (i, !c)
            } else {
                let d = query(a0.citer().map(|j| if j == c_idx { i } else { j }));
                if d == c { (i, c) } else { (i, !c) }
            }
        }))
        .sorted()
        .map(|t| t.1)
        .collect_vec();
    println!(
        "! {}",
        ans.citer().map(|a| if a { "R" } else { "B" }).join("")
    );
}
