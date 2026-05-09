fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    };

    let ans = iproduct!((0..=h).tuple_combinations(), (0..=w).tuple_combinations())
        .filter(|&((h0, h1), (w0, w1))| {
            iproduct!(0..h1 - h0, 0..w1 - w0)
                .all(|(i, j)| s[h0 + i][w0 + j] == s[h1 - 1 - i][w1 - 1 - j])
        })
        .count();

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
