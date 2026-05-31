fn main() {
    input! {
        t: usize,
        cases: [(i64, i64, i64, i64, i64, i64); t],
    };

    cases
        .into_iter()
        .map(|(x1, y1, r1, x2, y2, r2)| {
            let d2 = ((x1 - x2).pow(2) + (y1 - y2).pow(2));

            if d2 <= max(r1, r2).pow(2) {
                d2 >= (r1 - r2).pow(2)
            } else {
                d2 <= (r1 + r2).pow(2)
            }
        })
        .for_each(|ans| {
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        })
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
