fn main() {
    input! {
        t: usize,
        cases: [(usize, [(Usize1, usize)]); t],
    };

    cases
        .into_iter()
        .map(|(n, lr)| {
            let k = lr.iter().copied().map(|(l, r)| r - l).max().unwrap();

            (1..=k).cycle().take(n).collect::<Vec<_>>()
        })
        .for_each(|ans| {
            println!("{}", ans.iter().join(" "));
        });
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
