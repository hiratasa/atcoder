fn main() {
    input! {
        cases: [[i64]],
    };

    cases
        .into_iter()
        .map(|a| {
            let n = a.len();

            // eprintln!("##");
            let (z, c, k, l) = a.into_iter().fold((0, 0i64, 0, 0), |(z, c, k, l), x| {
                // eprintln!("#{z} {c} {k} {l}");
                if c <= x {
                    let r = x - c;

                    let (z, r) = if z < 0 {
                        (min(z + r, 0), max(0, z + r))
                    } else {
                        (z, r)
                    };

                    let l = l + 1;

                    let (c, r) = (c + r / (k + l), r % (k + l));

                    let (c, k, l) = if r <= k {
                        (c, k - r, l + r)
                    } else {
                        (c + 1, (k + l) - (r - k), r - k)
                    };

                    (z, c, k, l)
                } else {
                    (z - (c - x), c, k, l + 1)
                }
            });
            // eprintln!("#{z} {c} {k} {l}");

            z >= 0
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
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
