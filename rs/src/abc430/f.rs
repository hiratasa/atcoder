fn main() {
    input! {
        t: usize,
        cases: [(usize, Chars); t],
    };

    cases
        .into_iter()
        .map(|(n, s)| {
            let t0 = once(('L', 0))
                .chain(s.iter().scan(('L', 0), |(d, k), &c| {
                    if *d == c {
                        *k += 1;
                    } else {
                        *d = c;
                        *k = 1;
                    }

                    Some((*d, *k))
                }))
                .collect::<Vec<_>>();
            let t1 = once(('L', 0))
                .chain(s.iter().rev().scan(('L', 0), |(d, k), &c| {
                    if *d == c {
                        *k += 1;
                    } else {
                        *d = c;
                        *k = 1;
                    }

                    Some((*d, *k))
                }))
                .collect::<Vec<_>>()
                .into_iter()
                .rev();

            let mut t = izip!(t0, t1)
                .map(|((d0, k0), (d1, k1))| match (d0, d1) {
                    ('L', 'L') => (k1, n - k0),
                    ('L', 'R') => (0, n - (k0 + k1)),
                    ('R', 'L') => (k0 + k1, n),
                    ('R', 'R') => (k0, n - k1),
                    _ => unreachable!(),
                })
                .fold(vec![0i64; n + 1], |mut t, (i, j)| {
                    t[i] += 1;
                    t[j] -= 1;
                    t
                });

            for i in 0..n {
                t[i + 1] += t[i];
            }

            t.truncate(n);

            t
        })
        .for_each(|ans| {
            println!("{}", ans.iter().join(" "));
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
