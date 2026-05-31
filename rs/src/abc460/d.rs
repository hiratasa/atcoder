fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    };

    let t = (0..h)
        .map(|i| {
            (0..w)
                .map(|j| {
                    s[i][j] == '.'
                        && iproduct!([-1, 0, 1], [-1, 0, 1])
                            .filter(|&(di, dj)| di != 0 || dj != 0)
                            .filter_map(|(di, dj)| {
                                Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                            })
                            .filter(|&(ni, nj)| ni < h && nj < w)
                            .any(|(ni, nj)| s[ni][nj] == '#')
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut q = VecDeque::new();
    let mut dists = vec![vec![usize::MAX; w]; h];
    for i in 0..h {
        for j in 0..w {
            if t[i][j] {
                q.push_back((i, j));
                dists[i][j] = 0;
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        iproduct!([-1, 0, 1], [-1, 0, 1])
            .filter(|&(di, dj)| di != 0 || dj != 0)
            .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
            .filter(|&(ni, nj)| ni < h && nj < w)
            .for_each(|(ni, nj)| {
                if dists[ni][nj] == usize::MAX {
                    dists[ni][nj] = dists[i][j] + 1;
                    q.push_back((ni, nj));
                }
            });
    }

    for i in 0..h {
        println!(
            "{}",
            (0..w)
                .map(|j| if dists[i][j] % 2 == 0 || dists[i][j] == usize::MAX {
                    '.'
                } else {
                    '#'
                })
                .join("")
        );
    }
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
