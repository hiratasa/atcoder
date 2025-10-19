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

#[allow(dead_code)]
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
    let (n, m, q) = read_tuple!(usize, usize, usize);
    let s = read_digits();
    let lr = read_vec(m, || read_tuple!(usize, usize));
    let x = read_col::<usize>(q);

    let b = (q as f64).sqrt() as usize;

    let mut s = once(1).chain(s).chain(once(1)).collect::<Vec<_>>();
    let n = n + 2;

    (0..q).group_by(|&i| i / b).into_iter().for_each(|(_, it)| {
        let queries = it.map(|i_query| x[i_query]).collect::<Vec<_>>();

        let marked = queries.citer().fold(vec![false; n + 1], |mut marked, i| {
            marked[i] = true;
            marked
        });

        let targets = (0..n)
            .filter(|&i| i == 0 || i == n - 1 || marked[i])
            .collect::<Vec<_>>();

        let idxs = targets
            .citer()
            .enumerate()
            .fold(vec![None; n], |mut idxs, (idx, i)| {
                idxs[i] = Some(idx);
                idxs
            });
        let idxs_map = targets
            .citer()
            .enumerate()
            .map(|(idx, i)| (i, idx))
            .collect::<BTreeMap<_, _>>();

        let k = targets.len();

        let masked = (0..n)
            .map(|i| if marked[i] { 0 } else { s[i] })
            .collect::<Vec<_>>();

        let sums = once(0)
            .chain(s.citer())
            .cumsum::<usize>()
            .collect::<Vec<_>>();
        let masked_sums = once(0)
            .chain(masked.citer())
            .cumsum::<usize>()
            .collect::<Vec<_>>();

        let (score, mut counts) = lr.citer().fold(
            (0, vec![vec![0; k + 1]; k]),
            |(score, mut counts), (l, r)| {
                let r = r + 1;

                if masked_sums[l] < masked_sums[r] {
                    (score + 1, counts)
                } else {
                    let ll = *idxs_map.range(l..).next().unwrap().1;
                    let rr = idxs_map.range(r..).next().map_or(k, |(_, &rr)| rr);

                    counts[ll][rr] += 1;

                    (score + (sums[l] < sums[r]) as usize, counts)
                }
            },
        );

        for i in 0..k {
            for j in 1..=k {
                counts[i][j] += counts[i][j - 1];
            }
        }
        for i in 1..k {
            for j in 0..=k {
                counts[i][j] += counts[i - 1][j];
            }
        }

        let ones = targets
            .citer()
            .filter(|&i| s[i] > 0)
            .map(|i| idxs[i].unwrap())
            .collect::<BTreeSet<_>>();

        queries
            .citer()
            .scan((score, ones), |(score, ones), query| {
                let query = query;

                let idx = idxs[query].unwrap();

                if s[query] == 0 {
                    let prev = *ones.range(..idx).next_back().unwrap();
                    let next = *ones.range(idx + 1..).next().unwrap();

                    // [prev+1, idx] x [idx+1, next]
                    let c = counts[idx][next] + counts[prev][idx]
                        - counts[prev][next]
                        - counts[idx][idx];

                    *score += c;

                    s[query] = 1;
                    ones.insert(idx);
                } else {
                    s[query] = 0;
                    ones.remove(&idx);

                    let prev = *ones.range(..idx).next_back().unwrap();
                    let next = *ones.range(idx + 1..).next().unwrap();

                    // [prev+1, idx] x [idx+1, next]
                    let c = counts[idx][next] + counts[prev][idx]
                        - counts[prev][next]
                        - counts[idx][idx];

                    *score -= c;
                }

                Some(*score)
            })
            .for_each(|ans| {
                println!("{}", ans);
            });
    });
}
