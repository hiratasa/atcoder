#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
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
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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

fn dfs(
    idxs: &[Vec<usize>],
    pair: &mut [Option<usize>],
    pair2: &mut [Option<usize>],
    used: &mut [bool],
    v: usize,
    offset: usize,
) -> bool {
    used[v - offset] = true;

    for &u in &idxs[v] {
        if pair2[u].map_or(true, |w| {
            !used[w - offset] && dfs(idxs, pair, pair2, used, w, offset)
        }) {
            pair2[u] = Some(v);
            pair[v] = Some(u);
            return true;
        }
    }

    false
}

fn main() {
    let n = read::<usize>();
    let s = read_vec(n, || read_str());

    let l = s.iter().map(|ss| ss.len()).min().unwrap();

    for c in b'a'..=b'z' {
        let c = c as char;

        let idxs = s
            .iter()
            .enumerate()
            .fold(vec![vec![]; l + n], |mut idxs, (idx, ss)| {
                ss.citer()
                    .positions(|cc| cc == c)
                    .filter(|&pos| pos < l + n)
                    .for_each(|pos| {
                        idxs[pos].push(idx);
                    });
                idxs
            });

        let mut pair = vec![None; l + n];
        let mut pair2 = vec![None; n];

        let mut m = 0;
        for pos in 0..l {
            let m0 = m;

            if pos > 0 {
                if let Some(v) = pair[pos - 1] {
                    pair[pos - 1] = None;
                    pair2[v] = None;
                    m -= 1;
                }
            }

            let limit = if pos == 0 { n } else { m0 + 1 - m };
            m += (0..n)
                .scan(vec![], |used, i| {
                    if used.is_empty() {
                        used.resize(n, false);
                    }
                    if pair[pos + i].is_none()
                        && dfs(&idxs, &mut pair, &mut pair2, used, pos + i, pos)
                    {
                        used.clear();
                        Some(Some(()))
                    } else {
                        Some(None)
                    }
                })
                .flatten()
                .take(limit)
                .count();

            if m == n {
                println!("YES");
                return;
            }
        }
    }

    println!("NO");
}
