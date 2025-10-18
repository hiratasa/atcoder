fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
    };

    let b = a.into_iter().cycle().take(2 * n).collect::<Vec<_>>();
    let sums = once(0)
        .chain(b.iter().copied())
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    (0..q)
        .scan(0, |i0, _| {
            input! {
                t: usize,
            };

            if t == 1 {
                input! {
                    c: usize,
                };

                *i0 += c;
                *i0 %= n;

                Some(None)
            } else {
                input! {
                    l: Usize1, r: usize,
                };

                let l = l + *i0;
                let r = r + *i0;

                Some(Some(sums[r] - sums[l]))
            }
        })
        .flatten()
        .for_each(|ans| println!("{ans}"));
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
