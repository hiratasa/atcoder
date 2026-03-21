fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
        queries: [[Usize1]; q],
    };

    let t = a
        .iter()
        .copied()
        .enumerate()
        .sorted_by_key(|&(i, x)| x)
        .collect::<Vec<_>>();

    queries
        .into_iter()
        .map(|query| {
            t.iter()
                .copied()
                .find(|&(i, _)| !query.contains(&i))
                .unwrap()
                .1
        })
        .for_each(|ans| println!("{ans}"));
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
