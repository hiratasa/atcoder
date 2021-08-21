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
use itertools::{chain, iproduct, iterate, izip, Itertools};
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

fn z_algorithm<T: std::cmp::Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    let mut z = vec![0; n];
    z[0] = n;

    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        // assert!(s[l..r] == s[0..r - l]);
        if i < r && z[i - l] < r - i {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i..r] (=s[i-l..r-l]) = s[0..r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = std::cmp::max(i, r);
            while r < n && s[r] == s[r - l] {
                r += 1;
            }
            z[i] = r - l;
        }
    }

    z
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
    let (n, k) = read_tuple!(usize, usize);

    let s = read_vec(n, || read_str());

    let (dp0, _) = s.iter().enumerate().rev().fold(
        (vec![bitset!(k + 1, 0); n], bitset!(k + 1, 1)),
        |(mut dp0, mut r), (i, ss)| {
            dp0[i] = &r << ss.len();
            r |= &dp0[i];

            (dp0, r)
        },
    );

    let (ans, _) = izip!(s, dp0).fold(
        (vec![], vec![0]),
        |(mut ans, idxs): (Vec<char>, Vec<usize>), (ss, bs)| {
            let (sa, rank) = suffix_array(&ss);
            let lcp = lcp_array(&ss, &sa, &rank);
            let z = z_algorithm(&chain(ss.citer(), ans.citer()).collect::<Vec<char>>());

            let mut idxs2 = vec![];
            let (pos, idxs3) = idxs
                .citer()
                .filter(|&idx| bs[k - idx])
                .filter_map(|idx| {
                    let m = if idx == ans.len() {
                        0
                    } else {
                        min(z[ss.len() + idx], ss.len())
                    };

                    assert!(m == ss.len() || idx + m == ans.len() || ss[m] != ans[idx + m]);

                    if m < ss.len() {
                        if idx + m == ans.len() || ss[m] < ans[idx + m] {
                            Some((idx + m, rank[m], m))
                        } else {
                            None
                        }
                    } else {
                        idxs2.push(idx + m);
                        None
                    }
                })
                .sorted()
                .try_fold(
                    (std::usize::MAX, vec![], 0),
                    |(pos, idxs3, prev_r), (p, r, m)| {
                        if pos == std::usize::MAX {
                            return Ok((p, vec![p + ss.len() - m], r));
                        }

                        if pos != p {
                            return Err((pos, idxs3));
                        }

                        let prev_len = *idxs3.last().unwrap() - pos;
                        let len = lcp[prev_r..r].citer().min().unwrap();

                        if prev_len == len {
                            Ok((pos, pushed!(idxs3, pos + ss.len() - m), r))
                        } else {
                            Err((pos, idxs3))
                        }
                    },
                )
                .map_or_else(|(pos, idxs3)| (pos, idxs3), |(pos, idxs3, _)| (pos, idxs3));
            if pos != std::usize::MAX {
                let m = ss.len() - (*idxs3.last().unwrap() - pos);

                ans.resize_with(pos, || unreachable!());
                ans.extend(&ss[m..]);

                let idxs = chain(idxs, idxs2)
                    .filter(|&idx| idx <= pos)
                    .chain(idxs3)
                    .sorted()
                    .dedup()
                    .collect::<Vec<_>>();
                (ans, idxs)
            } else {
                let idxs = chain(idxs, idxs2).sorted().dedup().collect::<Vec<_>>();
                (ans, idxs)
            }
        },
    );

    println!("{}", ans.citer().join(""));
}
