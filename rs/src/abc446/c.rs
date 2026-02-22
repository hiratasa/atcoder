fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, d: usize,
            a: [usize; n],
            b: [usize; n],
        };

        let mut q = VecDeque::new();
        for (i, (x, y)) in izip!(a, b).enumerate() {
            q.extend(repeat_n(i, x));
            for _ in 0..y {
                q.pop_front();
            }

            while let Some(&j) = q.front()
                && i - j >= d
            {
                q.pop_front();
            }
        }

        println!("{}", q.len());
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
