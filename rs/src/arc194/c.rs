fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [i64; n],
    };

    let idxs = (0..n).sorted_by_key(|&i| c[i]).collect::<Vec<_>>();

    let a = idxs.iter().map(|&i| a[i]).collect::<Vec<_>>();
    let b = idxs.iter().map(|&i| b[i]).collect::<Vec<_>>();
    let c = idxs.iter().map(|&i| c[i]).collect::<Vec<_>>();

    let sums1 = once(0)
        .chain((0..n).scan(0, |s, i| {
            if a[i] == 1 && b[i] == 1 {
                Some(*s * c[i])
            } else if a[i] == 1 || b[i] == 1 {
                *s += 1;
                Some(0)
            } else {
                Some(0)
            }
        }))
        .cumsum::<i64>()
        .collect::<Vec<_>>();

    let sums2 = once(0)
        .chain((0..n).scan(0, |s, i| {
            if a[i] == 1 || b[i] == 1 {
                *s += c[i];
                if a[i] == 1 && b[i] == 1 {
                    *s += c[i];
                    Some(*s - c[i])
                } else {
                    Some(0)
                }
            } else {
                Some(0)
            }
        }))
        .cumsum::<i64>()
        .collect::<Vec<_>>();

    let sumsa = once(0)
        .chain((0..n).scan(0, |s, i| {
            if a[i] == 1 {
                *s += c[i];
                if b[i] == 0 {
                    Some(*s - c[i])
                } else {
                    Some(0)
                }
            } else {
                Some(0)
            }
        }))
        .cumsum::<i64>()
        .collect::<Vec<_>>();

    let sumsb = once(0)
        .chain((0..n).scan(0, |s, i| {
            if b[i] == 1 {
                *s += c[i];
                if a[i] == 0 {
                    Some(*s)
                } else {
                    Some(0)
                }
            } else {
                Some(0)
            }
        }))
        .cumsum::<i64>()
        .collect::<Vec<_>>();

    let ans = (0..=n)
        .filter(|&i| i == 0 || a[i - 1] == 1 && b[i - 1] == 1)
        // .inspect(|&i| eprint!("{i}: "))
        .map(|i| sumsa[n] + sumsb[n] + sums1[i] + (sums2[n] - sums2[i]))
        // .inspect(|&x| eprintln!("{x}"))
        .min()
        .unwrap();

    println!("{ans}");
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
