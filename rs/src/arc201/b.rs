fn main() {
    input! {
        t: usize,
    };

    (0..t)
        .map(|_| {
            input! {
                n: usize, w: usize,
                xy: [(usize, usize); n],
            };

            let mut items = xy.into_iter().fold(vec![vec![]; 60], |mut items, (x, y)| {
                items[x].push(y);
                items
            });

            let (ans, _) = (0..60).fold((0, vec![]), |(ans, mut items2), i| {
                items2.extend(take(&mut items[i]));
                items2.sort();

                let ans = if w & (1 << i) != 0 && !items2.is_empty() {
                    ans + items2.pop().unwrap()
                } else {
                    ans
                };

                if items2.len() % 2 == 0 {
                    for j in 0..items2.len() / 2 {
                        items2[j] = items2[2 * j] + items2[2 * j + 1];
                    }
                    items2.truncate(items2.len() / 2);
                } else {
                    for j in 0..items2.len() / 2 {
                        items2[j + 1] = items2[2 * j + 1] + items2[2 * j + 2];
                    }
                    items2.truncate(items2.len() / 2 + 1);
                }

                (ans, items2)
            });

            ans
        })
        .for_each(|ans| println!("{ans}"));
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
