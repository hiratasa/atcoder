fn main() {
    input! {
        t: usize,
        cases: [[usize]; t],
    };

    cases
        .into_iter()
        .map(|a| {
            let n = a.len();
            let o = a.iter().copied().filter(|&x| x % 2 == 1).count();
            let e = n - o;

            let x0 = if e >= 2 {
                let b = a
                    .iter()
                    .copied()
                    .filter(|&x| x % 2 == 0)
                    .sorted()
                    .rev()
                    .take(2)
                    .sum::<usize>();

                b + min(2 * o, 2 * e - 2)
            } else {
                0
            };

            let x1 = if e >= 1 && o >= 1 {
                let b = a.iter().copied().filter(|&x| x % 2 == 0).max().unwrap()
                    + a.iter().copied().filter(|&x| x % 2 == 1).max().unwrap();

                b + min(2 * o - 1, 2 * e - 1)
            } else {
                0
            };

            let x2 = if o >= 2 {
                let b = a
                    .iter()
                    .copied()
                    .filter(|&x| x % 2 == 1)
                    .sorted()
                    .rev()
                    .take(2)
                    .sum::<usize>();

                b + min(2 * e, 2 * o - 2)
            } else {
                0
            };

            a.iter().copied().sum::<usize>() - max(max(x0, x1), x2) / 2
        })
        .for_each(|ans| {
            println!("{ans}");
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
