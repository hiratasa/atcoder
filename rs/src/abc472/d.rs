fn main() {
    input! {
        h: usize, w: usize, k: usize,
        s: [Chars; h],
    };

    let has_boms_in_row = (0..h)
        .map(|i| (0..w).any(|j| s[i][j] == '#'))
        .collect::<Vec<_>>();
    let has_boms_in_col = (0..w)
        .map(|j| (0..h).any(|i| s[i][j] == '#'))
        .collect::<Vec<_>>();

    let mut dists = vec![vec![usize::MAX; w]; h];
    let mut q = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| !has_boms_in_row[i] && !has_boms_in_col[j])
        .inspect(|&(i, j)| dists[i][j] = 0)
        .collect::<VecDeque<_>>();

    while let Some((i, j)) = q.pop_front() {
        q.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
                .filter(|&(ni, nj)| ni < h && nj < w && s[ni][nj] == '.')
                .filter(|&(ni, nj)| {
                    if dists[i][j] + 1 < dists[ni][nj] {
                        dists[ni][nj] = dists[i][j] + 1;
                        true
                    } else {
                        false
                    }
                }),
        );
    }

    println!(
        "{}",
        iproduct!(0..h, 0..w)
            .filter(|&(i, j)| dists[i][j] <= k)
            .count()
    );
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
