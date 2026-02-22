fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let ans = a
        .into_iter()
        .scan(FxHashMap::default(), |map, x| {
            let t = map.get(&(x - 1)).copied().unwrap_or(0);
            let c = map.entry(x).or_insert(0);
            *c = max(*c, t + 1);
            Some(*c)
        })
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
