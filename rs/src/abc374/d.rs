fn main() {
    input! {
        n: usize, s: f64, t: f64,
        abcd: [[(f64, f64); 2]; n],
    };

    let ans = (0..n)
        .permutations(n)
        .cartesian_product(0..1 << n)
        .map(|(perm, x)| {
            once((0., 0.))
                .chain(
                    perm.iter()
                        .copied()
                        .zip((0..n).map(|i| x & (1 << i) > 0))
                        .flat_map(|(i, b)| {
                            if b {
                                [abcd[i][0], abcd[i][1]]
                            } else {
                                [abcd[i][1], abcd[i][0]]
                            }
                        }),
                )
                .tuple_windows()
                .enumerate()
                .map(|(i, (p, q))| {
                    let d = ((p.0 - q.0).powi(2) + (p.1 - q.1).powi(2)).sqrt();
                    if i % 2 == 0 { d / s } else { d / t }
                })
                .sum::<f64>()
        })
        .min_by(f64::total_cmp)
        .unwrap();

    println!("{ans}");
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
