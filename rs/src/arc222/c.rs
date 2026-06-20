fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            a: [[usize; n]; n],
        };

        let b = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        if j == 0 {
                            a[i][j + 1]
                        } else if j == n - 1 {
                            a[i][j - 1]
                        } else {
                            a[i][j - 1] + a[i][j + 1]
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut table0 = b.clone();
        for i in 1..n {
            for j in 0..n {
                if j == 0 {
                    table0[i][j] += table0[i - 1][j + 1];
                } else if j == n - 1 {
                    table0[i][j] += table0[i - 1][j - 1];
                } else {
                    table0[i][j] += min(table0[i - 1][j - 1], table0[i - 1][j + 1]);
                }
            }
        }
        let mut table1 = b.clone();
        for i in (0..n - 1).rev() {
            for j in 0..n {
                if j == 0 {
                    table1[i][j] += table1[i + 1][j + 1];
                } else if j == n - 1 {
                    table1[i][j] += table1[i + 1][j - 1];
                } else {
                    table1[i][j] += min(table1[i + 1][j - 1], table1[i + 1][j + 1]);
                }
            }
        }
        let mut table = table0.clone();
        for i in (0..n - 1).rev() {
            for j in 0..n {
                if j == 0 {
                    table[i][j] += table1[i + 1][j + 1];
                } else if j == n - 1 {
                    table[i][j] += table1[i + 1][j - 1];
                } else {
                    table[i][j] += min(table1[i + 1][j - 1], table1[i + 1][j + 1]);
                }
            }
        }

        for i in 0..n {
            println!(
                "{}",
                (0..n)
                    .map(|j| if j == 0 {
                        table[i][j + 1]
                    } else if j == n - 1 {
                        table[i][j - 1]
                    } else {
                        min(table[i][j - 1], table[i][j + 1])
                    })
                    .join(" ")
            );
        }
    }
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
