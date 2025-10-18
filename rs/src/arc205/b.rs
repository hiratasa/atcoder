fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let degs = uv.iter().fold(vec![0; n], |mut degs, &(u, v)| {
        degs[u] += 1;
        degs[v] += 1;

        degs
    });

    let ans = (0..n)
        .map(|i| {
            if (n - 1) % 2 == degs[i] % 2 {
                n - 1
            } else {
                n - 2
            }
        })
        .sum::<usize>()
        / 2;

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
