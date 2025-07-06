fn main() {
    input! {
        q: usize,
    };

    (0..q)
        .scan(VecDeque::new(), |a, _| {
            input! {
                ty: usize,
            };

            if ty == 1 {
                input! {
                    c: usize, x: usize,
                };

                a.push_back((c, x));

                Some(None)
            } else {
                input! {
                    mut k: usize,
                };

                let mut s = 0;
                while k > 0 {
                    let (c, x) = a.pop_front().unwrap();

                    if c <= k {
                        k -= c;
                        s += c * x;
                    } else {
                        s += k * x;
                        a.push_front((c - k, x));
                        break;
                    }
                }

                Some(Some(s))
            }
        })
        .flatten()
        .for_each(|ans| {
            println!("{ans}");
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
