fn main() {
    input! {
        h: usize, w: usize, n: usize,
        xy: [(Usize1, Usize1); n],
        q: usize,
        queries: [(usize, Usize1); q],
    };

    let by_x = xy
        .iter()
        .copied()
        .fold(vec![vec![]; h], |mut by_x, (x, y)| {
            by_x[x].push(y);
            by_x
        });
    let by_y = xy
        .iter()
        .copied()
        .fold(vec![vec![]; w], |mut by_y, (x, y)| {
            by_y[y].push(x);
            by_y
        });

    queries
        .into_iter()
        .scan((by_x, by_y), |(by_x, by_y), (ty, i)| {
            if ty == 1 {
                let ys = take(&mut by_x[i]);

                Some(ys.into_iter().filter(|&y| !by_y[y].is_empty()).count())
            } else {
                let xs = take(&mut by_y[i]);

                Some(xs.into_iter().filter(|&x| !by_x[x].is_empty()).count())
            }
        })
        .for_each(|ans| println!("{ans}"))
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
