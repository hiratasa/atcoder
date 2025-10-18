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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
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

fn suffix_array<T: Ord>(s: &[T]) -> (Vec<usize>, Vec<usize>) {
    let n = s.len();
    // 同じ文字間では添え字の逆順に並べる
    let sa0 = (0..n)
        .sorted_by_key(|&i| (&s[i], std::cmp::Reverse(i)))
        .collect_vec();
    let (rank0, max_rank) = sa0
        .iter()
        .group_by(|&&i| &s[i])
        .into_iter()
        .enumerate()
        .fold((vec![0; n], 0), |(mut rank, _), (r, (_, it))| {
            for &idx in it {
                rank[idx] = r;
            }
            (rank, r)
        });

    iterate(2, |len| len * 2)
        .take_while(|&len| len / 2 < n)
        .try_fold(
            (sa0, rank0, max_rank),
            |(prev_sa, prev_rank, prev_max_rank), len| {
                let counts =
                    prev_rank
                        .iter()
                        .fold(vec![0; prev_max_rank + 1], |mut counts, &idx| {
                            counts[idx] += 1;
                            counts
                        });
                let cum_counts = counts.iter().cumsum::<usize>().collect::<Vec<_>>();

                // prev_saは各suffixのlen/2文字の部分の昇順になっており、
                // かつlen/2文字の部分が同じときは添え字の降順に並んでいる
                // => n-len/2より大きいものはprev_saから変化なし
                //    それ以外の部分は前半len/2文字分で安定ソートする
                let sa = prev_sa
                    .iter()
                    .copied()
                    .filter_map(|i| i.checked_sub(len / 2))
                    .rev()
                    .fold(
                        (prev_sa.clone(), cum_counts),
                        |(mut sa, mut cum_counts), i| {
                            cum_counts[prev_rank[i]] -= 1;
                            sa[cum_counts[prev_rank[i]]] = i;
                            (sa, cum_counts)
                        },
                    )
                    .0;

                let to_key = |i: usize| (prev_rank.get(i), prev_rank.get(i + len / 2));
                let (rank, max_rank) = sa
                    .iter()
                    .group_by(|&&i| to_key(i))
                    .into_iter()
                    .enumerate()
                    .fold((vec![0; n], 0), |(mut rank, _), (r, (_, it))| {
                        for &idx in it {
                            rank[idx] = r;
                        }
                        (rank, r)
                    });

                if max_rank == n - 1 {
                    // これ以上の比較は不要
                    Err((sa, rank))
                } else {
                    Ok((sa, rank, max_rank))
                }
            },
        )
        // n=1のときはerrにならないので注意
        .map_or_else(|(sa, rank)| (sa, rank), |(sa, rank, _)| (sa, rank))
}

#[allow(dead_code)]
fn lcp_array(s: &[char], sa: &[usize], sa_rank: &[usize]) -> Vec<usize> {
    let n = sa_rank.len();

    let mut lcp = vec![0; n - 1];

    let mut l = 0;
    for i in 0..n {
        if sa_rank[i] == 0 {
            continue;
        }

        let i1 = i;
        let i2 = sa[sa_rank[i] - 1];
        while i1 + l < n && i2 + l < n && s[i1 + l] == s[i2 + l] {
            l += 1;
        }

        lcp[sa_rank[i] - 1] = l;
        l = l.checked_sub(1).unwrap_or(0);
    }

    lcp
}

fn main() {
    let n = read::<usize>();
    let s = read_str();
    let t = read_str();

    let u = s
        .citer()
        .chain(s.citer())
        .chain(t.citer())
        .chain(t.citer())
        .collect::<Vec<_>>();

    let (sa, rank) = suffix_array(&u);

    let lcp = lcp_array(&u, &sa, &rank);

    let ans0 = sa
        .citer()
        .scan(0, |p, idx| {
            if idx < n {
                *p += 1;
                Some(0)
            } else if 2 * n <= idx && idx < 3 * n {
                Some(*p)
            } else {
                Some(0)
            }
        })
        .sum::<usize>();

    let ans1 = izip!(sa, once(0).chain(lcp.citer()))
        .scan(0, |p, (idx, l)| {
            if l < n {
                *p = 0;
            }

            if idx < n {
                Some(*p)
            } else if 2 * n <= idx && idx < 3 * n {
                *p += 1;
                Some(0)
            } else {
                Some(0)
            }
        })
        .sum::<usize>();

    let ans = ans0 + ans1;

    println!("{}", ans);
}
