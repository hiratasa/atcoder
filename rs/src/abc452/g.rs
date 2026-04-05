fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let b = a
        .iter()
        .copied()
        .chunk_by(|&x| x)
        .into_iter()
        .map(|(x, it)| (x, it.count()))
        .collect::<Vec<_>>();

    let c = b
        .iter()
        .copied()
        .flat_map(|(x, len)| {
            if x > len {
                [Some(10), None, None]
            } else if x == len {
                [Some(x), None, None]
            } else {
                [Some(x), Some(10), Some(x)]
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let (sa, rank) = suffix_array(&c);
    let lcp = lcp_array(&c, &sa, &rank);

    let mut maxlen = c
        .iter()
        .copied()
        .enumerate()
        .rev()
        .scan(c.len(), |l, (i, x)| {
            if x == 10 {
                *l = i;
            }
            Some(*l - i)
        })
        .collect::<Vec<_>>();
    maxlen.reverse();

    // eprintln!("{c:?}");
    // eprintln!("{sa:?}");
    // eprintln!("{lcp:?}");
    // eprintln!("{maxlen:?}");

    let mut ans = maxlen[sa[0]];
    for (i, (_idx0, idx1)) in sa.iter().copied().tuple_windows().enumerate() {
        let maxlen1 = maxlen[idx1];
        let l = lcp[i];

        ans += maxlen1.saturating_sub(l);
    }

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

fn suffix_array<T: Ord>(s: &[T]) -> (Vec<usize>, Vec<usize>) {
    let n = s.len();
    // 同じ文字間では添え字の逆順に並べる
    let sa0 = (0..n)
        .sorted_by_key(|&i| (&s[i], std::cmp::Reverse(i)))
        .collect_vec();
    let (rank0, max_rank) = sa0.chunk_by(|&i, &j| s[i] == s[j]).enumerate().fold(
        (vec![0; n], 0),
        |(mut rank, _), (r, chunk)| {
            for &idx in chunk {
                rank[idx] = r;
            }
            (rank, r)
        },
    );

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
                    .chunk_by(|&i, &j| to_key(i) == to_key(j))
                    .enumerate()
                    .fold((vec![0; n], 0), |(mut rank, _), (r, chunk)| {
                        for &idx in chunk {
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

fn lcp_array(s: &[usize], sa: &[usize], sa_rank: &[usize]) -> Vec<usize> {
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
