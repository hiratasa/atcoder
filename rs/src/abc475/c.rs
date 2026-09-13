fn main() {
    input! {
        n: usize, s: Usize1, l: usize,
        a: [usize; n - 1],
    };

    let b = once(0).chain(a).cumsum::<usize>().collect::<Vec<_>>();

    let ans = iproduct!(0..=s, s..n)
        .filter(|&(x, y)| {
            let l0 = b[s] - b[x];
            let l1 = b[y] - b[s];

            l0 + l1 + min(l0, l1) <= l
        })
        .map(|(x, y)| (y - s) + (s - x) + 1)
        .max()
        .unwrap();

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
