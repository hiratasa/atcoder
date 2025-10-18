fn main() {
    input! {
        h: usize, w: usize, q: usize,
        rc: [(Usize1, Usize1); q],
    };

    let rows = (0..h)
        .map(|_| (0..w).collect::<BTreeSet<_>>())
        .collect::<Vec<_>>();
    let cols = (0..w)
        .map(|_| (0..h).collect::<BTreeSet<_>>())
        .collect::<Vec<_>>();

    let ans = h * w
        - rc.into_iter()
            .scan((rows, cols), |(rows, cols), (r, c)| {
                if rows[r].contains(&c) {
                    rows[r].remove(&c);
                    cols[c].remove(&r);

                    Some(1)
                } else {
                    let left = if let Some(&cc) = rows[r].range(..c).next_back() {
                        rows[r].remove(&cc);
                        cols[cc].remove(&r);
                        1
                    } else {
                        0
                    };

                    let right = if let Some(&cc) = rows[r].range(c..).next() {
                        rows[r].remove(&cc);
                        cols[cc].remove(&r);
                        1
                    } else {
                        0
                    };

                    let top = if let Some(&rr) = cols[c].range(..r).next_back() {
                        rows[rr].remove(&c);
                        cols[c].remove(&rr);
                        1
                    } else {
                        0
                    };

                    let bottom = if let Some(&rr) = cols[c].range(r..).next() {
                        rows[rr].remove(&c);
                        cols[c].remove(&rr);
                        1
                    } else {
                        0
                    };

                    Some(left + right + top + bottom)
                }
            })
            .sum::<usize>();

    println!("{ans}");
}

use std::collections::BTreeSet;
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
