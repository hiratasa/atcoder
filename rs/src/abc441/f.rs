fn main() {
    input! {
        n: usize, m: usize,
        pv: [(usize, usize); n],
    };

    let t0 = once(vec![0; m + 1])
        .chain(pv.iter().scan(vec![0; m + 1], |a, &(p, v)| {
            for i in (0..=m - p).rev() {
                a[i + p] = max(a[i + p], a[i] + v);
            }

            Some(a.clone())
        }))
        .collect::<Vec<_>>();
    let t1 = once(vec![0; m + 1])
        .chain(pv.iter().rev().scan(vec![0; m + 1], |a, &(p, v)| {
            for i in (0..=m - p).rev() {
                a[i + p] = max(a[i + p], a[i] + v);
            }

            Some(a.clone())
        }))
        .collect::<Vec<_>>();

    let z = t0[n][m];
    println!(
        "{}",
        izip!(t0, pv, t1.into_iter().rev().skip(1))
            .map(|(a, (p, v), b)| {
                let ok0 = (0..=m).map(|i| a[i] + b[m - i]).any(|x| x == z);
                let ok1 = (0..=m - p).map(|i| a[i] + b[m - p - i] + v).any(|x| x == z);

                match (ok0, ok1) {
                    (true, true) => "B",
                    (true, false) => "C",
                    (false, true) => "A",
                    (false, false) => unreachable!(),
                }
            })
            .join("")
    );
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
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
use rustc_hash::{FxHashMap, FxHashSet};
