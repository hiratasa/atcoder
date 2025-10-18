#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{Readable, Source},
};

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
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
        bs
    }};
}

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

enum Digits {}

impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}

#[allow(dead_code)]
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
    input! {
        s: Chars,
        k: usize,
    };

    let (sa, rank) = suffix_array(&s);
    let lcp = lcp_array(&s, &sa, &rank);

    let n = s.len();
    let ans = izip!(
        sa.citer().chain(once(n)).tuple_windows(),
        lcp.citer().chain(once(0))
    )
    .filter(|&((i, j), l)| n - i >= k && l < k)
    .count();

    println!("{ans}");
}
