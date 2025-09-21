fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            h: usize, w: usize,
            s: [Chars; h],
        };

        let s = s
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .fold(0usize, |x, c| (x << 1) | if c == '#' { 1 } else { 0 })
            })
            .collect::<Vec<_>>();

        let mut init = vec![usize::MAX; 1 << w];
        init[0] = 0;
        let dp = s
            .iter()
            .enumerate()
            .take(h - 1)
            .fold(init, |prev, (i, &row)| {
                let mut next = vec![usize::MAX; 1 << w];
                for (x, p) in prev.into_iter().enumerate() {
                    let nrow = row & !x;

                    for y in 0..(1 << w) {
                        let nrow2 = s[i + 1] & !y;
                        let z = nrow & nrow2;
                        let is_valid = (0..w - 1).all(|j| (z >> j) & 3 != 3);

                        if is_valid {
                            next[y] = min(next[y], p.saturating_add(y.count_ones() as usize));
                        }
                    }
                }

                next
            });

        println!("{}", dp.iter().min().unwrap());
    }
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
