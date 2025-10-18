fn main() {
    input! {
        n: usize, m: usize, y: usize,
        a: [usize; n],
    };

    let mut t = (0..1 << n)
        .map(|bits| {
            (0..n)
                .filter(|&i| (bits >> i) & 1 == 1)
                .try_fold(1, |l, i| {
                    let g = gcd(l, a[i]);

                    let ll = l / g;

                    if ll <= y / a[i] {
                        Some(ll * a[i])
                    } else {
                        None
                    }
                })
                .map_or(0, |l| y / l)
        })
        .collect::<Vec<_>>();

    for i in 0..n {
        for bits in 0..1 << n {
            if (bits >> i) & 1 == 0 {
                t[bits] = t[bits] - t[bits ^ (1 << i)];
            }
        }
    }

    let ans = (0usize..1 << n)
        .filter(|&bits| bits.count_ones() == m as u32)
        .map(|bits| t[bits])
        .sum::<usize>();

    println!("{ans}");
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
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
