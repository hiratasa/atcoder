fn main() {
    input! {
        n: usize, m: usize,
        c: [usize; m],
        ab: [(Usize1, usize); n],
    };

    let ans = ab
        .iter()
        .copied()
        .sorted()
        .chunk_by(|&(a, _)| a)
        .into_iter()
        .map(|(a, it)| min(it.map(|(_, b)| b).sum::<usize>(), c[a]))
        .sum::<usize>();

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
