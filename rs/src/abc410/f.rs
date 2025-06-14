fn main() {
    input! {
        t: usize,
    };

    (0..t)
        .map(|_| {
            input! {
                h: usize, w: usize,
                s: [Chars; h],
            };

            let (s, h, w) = if h < w {
                (s, h, w)
            } else {
                let rotated = (0..w)
                    .map(|j| (0..h).map(|i| s[i][j]).collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                (rotated, w, h)
            };

            let sums = once(vec![0; w + 1])
                .chain((0..h).map(|i| {
                    once(0)
                        .chain(
                            s[i].iter()
                                .copied()
                                .map(|c| if c == '#' { 1 } else { -1 })
                                .cumsum::<i64>(),
                        )
                        .collect::<Vec<_>>()
                }))
                .scan(vec![0; w + 1], |sum, row| {
                    sum.iter_mut().zip(row.iter()).for_each(|(s, &r)| *s += r);
                    Some(sum.clone())
                })
                .collect::<Vec<_>>();

            (0..=h)
                .tuple_combinations()
                .map(|(i0, i1)| {
                    (1..=w)
                        .scan(once((0, 1)).collect::<FxHashMap<_, _>>(), |map, j| {
                            let x = sums[i1][j] - sums[i0][j];
                            let p = map.entry(x).or_insert(0);
                            *p += 1;
                            Some(*p - 1)
                        })
                        .sum::<i64>()
                })
                .sum::<i64>()
        })
        .for_each(|ans| {
            println!("{ans}");
        })
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
