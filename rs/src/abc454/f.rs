fn main() {
    input! {
        t: usize,
    };

    (0..t)
        .map(|_| {
            input! {
                n: usize, m: i64,
                a:[i64; n],
            };

            let b = once(0)
                .chain(a)
                .chain(once(0))
                .tuple_windows()
                .map(|(x, y)| (y - x).rem_euclid(m))
                .collect::<Vec<_>>();
            let c = if n % 2 == 0 {
                (0..n / 2)
                    .map(|i| b[i] + b[b.len() - 1 - i])
                    .chain(once(b[n / 2]))
                    .collect::<Vec<_>>()
            } else {
                (0..=n / 2)
                    .map(|i| b[i] + b[b.len() - 1 - i])
                    .collect::<Vec<_>>()
            };
            let d = c
                .into_iter()
                .map(|x| x.rem_euclid(m))
                .sorted()
                .collect::<Vec<_>>();

            let s = d.iter().copied().sum::<i64>();
            once(0)
                .chain(d.iter().copied().cumsum::<i64>())
                .enumerate()
                .map(|(i, t)| max(t, ((d.len() - i) as i64) * m - (s - t)))
                .min()
                .unwrap()
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
