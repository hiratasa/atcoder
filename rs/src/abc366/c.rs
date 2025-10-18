fn main() {
    input! {
        q: usize,
    };

    (0..q)
        .scan(FxHashMap::default(), |set, _| {
            input! {
                idx: usize,
            };

            match idx {
                1 => {
                    input! { x: usize };

                    *set.entry(x).or_insert(0) += 1;

                    Some(None)
                }
                2 => {
                    input! { x: usize };

                    *set.get_mut(&x).unwrap() -= 1;

                    if set[&x] == 0 {
                        set.remove(&x);
                    }

                    Some(None)
                }
                3 => Some(Some(set.len())),
                _ => unreachable!(),
            }
        })
        .flatten()
        .for_each(|ans| {
            println!("{ans}");
        });
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
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
