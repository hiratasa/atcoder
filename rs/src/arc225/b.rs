fn main() {
    input! {
        t: usize,
        cases: [[usize]; t],
    };

    cases
        .into_iter()
        .map(|a| {
            let n = a.len();

            let blocks = a
                .iter()
                .copied()
                .chunk_by(|&x| x)
                .into_iter()
                .filter_map(|(x, it)| if x == 0 { None } else { Some(it.count()) })
                .collect::<Vec<_>>();

            blocks.iter().any(|&x| x != 2)
        })
        .for_each(|ans| {
            if ans {
                println!("Alice");
            } else {
                println!("Bob")
            };
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
