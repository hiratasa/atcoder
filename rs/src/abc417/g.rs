fn main() {
    input! {
        q: usize,
        lrx: [(usize, usize, Usize1); q],
    };

    let sizes = lrx
        .iter()
        .fold(vec![1usize, 1usize], |mut sizes, &(l, r, _x)| {
            sizes.push(sizes[l].saturating_add(sizes[r]));
            sizes
        });

    let heavy = lrx
        .iter()
        .fold(vec![(0, 0, 0), (0, 1, 0)], |mut heavy, &(l, r, _x)| {
            if sizes[l] < sizes[r] {
                heavy.push((sizes[l], r, 0));
            } else {
                heavy.push((0, l, sizes[r]));
            }
            heavy
        });

    let mut heavies = vec![heavy];
    for i in 1..20 {
        heavies.push(
            (0..q + 2)
                .map(|j| {
                    let (ll, c, rr) = heavies[i - 1][j];
                    let (ll2, c2, rr2) = heavies[i - 1][c];

                    (ll.saturating_add(ll2), c2, rr.saturating_add(rr2))
                })
                .collect::<Vec<_>>(),
        );
    }

    let ans = lrx
        .iter()
        .enumerate()
        .map(|(idx, &(l, r, x))| {
            let mut v = idx + 2;
            let mut xx = x;
            while v > 1 {
                for i in (0..=v.ilog2() as usize).rev() {
                    let (ll, c, rr) = heavies[i][v];
                    if ll <= xx && xx < ll.saturating_add(sizes[c]) {
                        assert!(c < v || v <= 1);
                        v = c;
                        xx -= ll;
                    }
                }

                if v <= 1 {
                    break;
                }

                if xx < sizes[lrx[v - 2].0] {
                    v = lrx[v - 2].0;
                } else {
                    xx -= sizes[lrx[v - 2].0];
                    v = lrx[v - 2].1;
                }
            }

            assert_eq!(xx, 0);

            v
        })
        .collect::<Vec<_>>();

    println!("{}", ans.iter().join("\n"));
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
