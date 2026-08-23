fn main() {
    input! {
        n: usize, m: usize, k: usize,
        a: [usize; n],
    };

    println!(
        "{}",
        a.iter()
            .copied()
            .scan((VecDeque::<usize>::new(), 0), |(q, s), x| {
                if q.len() + 1 > m {
                    *s -= q.pop_front().unwrap();
                }

                if *s + x <= k {
                    q.push_back(x);
                    *s += x;
                    Some(true)
                } else {
                    q.push_back(0);
                    Some(false)
                }
            })
            .map(|ans| if ans { "Yes" } else { "No" })
            .join("\n")
    );
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
