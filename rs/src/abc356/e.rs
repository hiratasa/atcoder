fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort();

    let b = a
        .into_iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(x, it)| (x, it.count()))
        .collect::<Vec<_>>();
    let sums = once(0)
        .chain(b.iter().map(|&(_, k)| k))
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    let ma = b[b.len() - 1].0;
    let ans = b
        .iter()
        .copied()
        .map(|(x, k)| {
            (1..=ma / x)
                .map(|i| {
                    let idx = b
                        .binary_search_by(|&(y, _)| y.cmp(&(i * x)).then(Ordering::Greater))
                        .unwrap_err();

                    sums[sums.len() - 1] - sums[idx]
                })
                .sum::<usize>()
                * k
                - k * (k + 1) / 2
        })
        .sum::<usize>();

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
