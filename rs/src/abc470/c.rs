fn main() {
    input! {
        n: usize, q: usize,
    };

    let mut nums = FxHashMap::default();
    let mut ans = 0;
    for _ in 0..q {
        input! {
            ty: usize,
        };

        if ty == 1 {
            input! {
                i: Usize1,
            };

            let x = nums.entry(i).or_insert(0);

            ans ^= *x;
            *x += 1;
            ans ^= *x;
        } else {
            nums.retain(|_, v| {
                ans ^= *v;
                *v -= 1;
                ans ^= *v;
                *v > 0
            });
        }

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
