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

use std::io::BufWriter;
use std::io::Write;

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
        let x = $x;
        let mut c = $c;
        c.push(x);
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
    let (n, k) = read_tuple!(usize, i64);
    let x = read_row::<i64>();
    let q: usize = read();
    let query = read_vec(q, || read_tuple!(usize, usize));

    let start = std::time::Instant::now();

    let x = once(-(1 << 50))
        .chain(x)
        .chain(once(1 << 50))
        .collect::<Vec<_>>();
    let nextr0 = x
        .citer()
        .rev()
        .scan(n + 1, |idx, xx| {
            while xx + k <= x[*idx - 1] {
                *idx -= 1;
            }
            Some(*idx as u32)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    let nextl0 = x
        .citer()
        .scan(0, |idx, xx| {
            while x[*idx + 1] + k <= xx {
                *idx += 1;
            }
            Some(*idx as u32)
        })
        .collect::<Vec<_>>();

    const B: usize = 18;
    // メモリアクセス高速化のためにusizeじゃなくてu32使う
    let nextr = (1..B).fold(vec![nextr0], |nextr, i| {
        pushed!(
            nextr,
            (0..=n + 1)
                .map(|j| { nextr[i - 1][nextr[i - 1][j] as usize] })
                .collect()
        )
    });
    let nextl = (1..B).fold(vec![nextl0], |nextl, i| {
        pushed!(
            nextl,
            (0..=n + 1)
                .map(|j| { nextl[i - 1][nextl[i - 1][j] as usize] })
                .collect()
        )
    });
    let cumr = (0..=n + 1).rev().fold(vec![0; n + 2], |mut cumr, i| {
        cumr[i] = cumr[nextr[0][i] as usize] + i;
        cumr
    });
    let cuml = (0..=n + 1).fold(vec![0; n + 2], |mut cuml, i| {
        cuml[i] = cuml[nextl[0][i] as usize] + i;
        cuml
    });
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    query
        .citer()
        .map(|(l, r)| {
            let (s, r2, l2) = (0..B).rev().fold((1, l, r), |(s, idx, idx2), i| {
                if nextr[i][idx] as usize <= r {
                    (
                        s + (1 << i),
                        nextr[i][idx] as usize,
                        nextl[i][idx2] as usize,
                    )
                } else {
                    (s, idx, idx2)
                }
            });
            (cuml[r] - cuml[nextl[0][l2] as usize]) - (cumr[l] - cumr[nextr[0][r2] as usize]) + s
        })
        .for_each(|ans| {
            writeln!(stdout, "{}", ans).unwrap();
        });

    eprintln!("{}ms", start.elapsed().as_millis());
}
