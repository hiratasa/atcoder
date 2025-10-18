fn main() {
    input! {
        n: usize,
        mut a: [[[usize; n]; n]; n],
        q: usize,
        queries: [[(Usize1, usize); 3]; q],
    };

    for i in 0..n {
        for j in 0..n {
            a[i][j].insert(0, 0);
        }
        a[i].insert(0, vec![0; n + 1]);
    }
    a.insert(0, vec![vec![0; n + 1]; n + 1]);

    for i in 0..=n {
        for j in 0..=n {
            for k in 0..n {
                a[i][j][k + 1] += a[i][j][k];
            }
        }
    }

    for i in 0..=n {
        for j in 0..n {
            for k in 0..=n {
                a[i][j + 1][k] += a[i][j][k];
            }
        }
    }

    for i in 0..n {
        for j in 0..=n {
            for k in 0..=n {
                a[i + 1][j][k] += a[i][j][k];
            }
        }
    }

    queries
        .into_iter()
        .map(|query| {
            (0..3)
                .map(|i| {
                    let (l, r) = query[i];

                    [(l, -1i64), (r, 1i64)]
                })
                .multi_cartesian_product()
                .map(|t| {
                    let sgn = t[0].1 * t[1].1 * t[2].1;
                    sgn * a[t[0].0][t[1].0][t[2].0] as i64
                })
                .sum::<i64>()
        })
        .for_each(|ans| {
            println!("{ans}");
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
