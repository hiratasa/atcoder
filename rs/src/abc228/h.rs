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

fn main() {
    let (n, x) = read_tuple!(usize, i64);
    let ac = read_vec(n, || read_tuple!(i64, i64));

    let t = ac
        .citer()
        .sorted()
        .group_by(|&(a, _)| a)
        .into_iter()
        .map(|(a, it)| (a, it.map(|(_, c)| c).sum::<i64>()))
        .collect::<Vec<_>>();

    let csum = once(0)
        .chain(t.citer().map(|(_, c)| c))
        .cumsum::<i64>()
        .collect::<Vec<_>>();
    let acsum = once(0)
        .chain(t.citer().map(|(a, c)| a * c))
        .cumsum::<i64>()
        .collect::<Vec<_>>();

    let m = t.len();
    let (dp, _, _) = t.citer().enumerate().fold(
        (vec![0; m + 1], vec![(0, 0)], 0),
        |(mut dp, mut s, mut idx), (i, (a, _c))| {
            // min{j=<i} dp[j] + a * (csum[i] - csum[j]) - (acsum[i] - acsum[j]) + X
            // = min{j=<i} [dp[j] - a * csum[j] + acsum[j]] + a * csum[i] - acsum[i] + X
            while idx + 1 < s.len() && s[idx].0 - a * s[idx].1 > s[idx + 1].0 - a * s[idx + 1].1 {
                idx += 1;
            }
            dp[i + 1] = s[idx].0 - a * s[idx].1 + a * csum[i] - acsum[i] + x;

            // eprintln!("{:?} {:?} {}", dp, s, idx);

            let (u, v) = (dp[i + 1] + acsum[i + 1], csum[i + 1]);
            while s.len() >= 2
                && (v - s[s.len() - 2].1) * (s[s.len() - 1].0 - s[s.len() - 2].0)
                    - (u - s[s.len() - 2].0) * (s[s.len() - 1].1 - s[s.len() - 2].1)
                    >= 0
            {
                s.pop();
                if idx >= s.len() {
                    idx -= 1;
                }
            }
            s.push((u, v));

            (dp, s, idx)
        },
    );
    let ans = dp[m];

    println!("{}", ans);
}
