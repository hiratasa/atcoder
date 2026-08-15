fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, m: usize,
            mut a: [usize; m],
        };
        a.resize(62, 0);

        let ans = (0..=61).rev().fold(0usize, |fixed, i| {
            let s = fixed + (1usize << i) - 1;

            let mut b = a.clone();
            for j in 0..=61 {
                if s & (1 << j) > 0 {
                    if b[j] > n {
                        if j + 1 <= 61 {
                            b[j + 1] += (b[j] + 1 - n) / 2;
                        } else {
                            return fixed + (1usize << i);
                        }
                    }
                } else {
                    if j + 1 <= 61 {
                        b[j + 1] += (b[j] + 1) / 2;
                    } else if b[j] > 0 {
                        return fixed + (1usize << i);
                    }
                }
            }

            fixed
        });

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
