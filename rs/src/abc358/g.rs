fn main() {
    input! {
        h: usize, w: usize, k: usize,
        s: (Usize1, Usize1),
        a: [[i64; w]; h],
    };

    let u = min(h * w, k);

    let mut init = vec![vec![i64::MIN; w]; h];
    init[s.0][s.1] = 0;
    let b = (0..u).fold(init, |prev, _| {
        (0..h)
            .map(|i| {
                (0..w)
                    .map(|j| {
                        a[i][j]
                            + [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]
                                .into_iter()
                                .filter_map(|(di, dj)| {
                                    Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                                })
                                .filter(|&(ni, nj)| ni < h && nj < w)
                                .map(|(ni, nj)| prev[ni][nj])
                                .max()
                                .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });

    let ans = iproduct!(0..h, 0..w)
        .map(|(i, j)| b[i][j] + (k - u) as i64 * a[i][j])
        .max()
        .unwrap();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
