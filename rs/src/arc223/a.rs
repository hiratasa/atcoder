fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, w: usize,
            wv: [(usize, i64); n],
        };

        let (a, b, _) =
            wv.iter()
                .copied()
                .rev()
                .fold((i64::MIN, 0, 0), |(a, b, ws), (weight, value)| {
                    if ws + weight <= w {
                        (max(a + value, b), b + value, ws + weight)
                    } else {
                        (a + value, b, ws)
                    }
                });

        let ans = max(a, b);

        println!("{ans}");
    }
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
