fn main() {
    input! {
        t: usize,
        cases: [[usize]; t],
    };

    cases
        .into_iter()
        .map(|a| {
            let n = a.len();

            if a.iter().copied().tuple_windows().any(|(x, y)| x < 2 * y) {
                None
            } else if a.iter().all(|&x| x == 0) {
                Some(0)
            } else {
                Some(
                    a.iter()
                        .copied()
                        .tuple_windows()
                        .map(|(x, y)| x - 2 * y)
                        .chain(once(1))
                        .max()
                        .unwrap(),
                )
            }
        })
        .for_each(|ans| {
            if let Some(ans) = ans {
                println!("{ans}");
            } else {
                println!("-1");
            }
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
