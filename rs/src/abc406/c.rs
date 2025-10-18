fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let t0 = (0..=n - 3)
        .filter(|&i| p[i] < p[i + 1] && p[i + 1] > p[i + 2])
        .collect::<Vec<_>>();
    let t1 = (0..=n - 3)
        .filter(|&i| p[i] > p[i + 1] && p[i + 1] < p[i + 2])
        .collect::<Vec<_>>();

    let ans = kmerge(
        [t0, t1]
            .into_iter()
            .enumerate()
            .map(|(idx, t)| t.into_iter().map(move |i| (i, idx))),
    )
    .chain(once((n - 2, 2)))
    .tuple_windows()
    .scan(0, |prev, ((i, k0), (j, k1), (l, _))| {
        let z = if k0 == 0 && k1 == 1 {
            (1 + i - *prev) * (l - j)
        } else {
            0
        };

        *prev = i + 1;

        Some(z)
    })
    .sum::<usize>();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
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
