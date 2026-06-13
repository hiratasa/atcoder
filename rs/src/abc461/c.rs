fn main() {
    input! {
        n: usize, k: usize, m: usize,
        cv: [(Usize1, usize); n],
    };

    let items = cv
        .into_iter()
        .sorted()
        .chunk_by(|&(c, _v)| c)
        .into_iter()
        .map(|(_c, it)| it.map(|(_, v)| v).sorted().rev().collect::<Vec<_>>())
        .sorted_by_key(|v| Reverse(v[0]))
        .collect::<Vec<_>>();

    let s0 = items[..m].iter().map(|v| v[0]).sum::<usize>();

    let s1 = chain(
        items[m..].iter().map(|v| &v[0]),
        items.iter().flat_map(|v| &v[1..]),
    )
    .sorted()
    .rev()
    .take(k - m)
    .sum::<usize>();

    println!("{}", s0 + s1);
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
