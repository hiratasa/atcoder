fn main() {
    input! {
        t: usize,
        cases: [(usize, Digits); t],
    };

    cases
        .into_iter()
        .map(|(n, s)| {
            iterate(s, |s| s.iter().copied().map(|x| 1 - x).collect::<Vec<_>>())
                .take(2)
                .map(|s| {
                    let from_left = once(0)
                        .chain(s.iter().copied().scan((0, 0), |(t, k), d| {
                            if d == 0 {
                                *k += 1;
                            } else {
                                *t += *k * 2 + 1;
                                *k = 0;
                            }
                            Some(*t)
                        }))
                        .collect::<Vec<_>>();
                    let from_right = once(0)
                        .chain(s.iter().copied().rev().scan((0, 0), |(t, k), d| {
                            if d == 0 {
                                *k += 1;
                            } else {
                                *t += *k * 2 + 1;
                                *k = 0;
                            }
                            Some(*t)
                        }))
                        .collect::<Vec<_>>();

                    izip!(from_left, from_right.into_iter().rev())
                        .map(|(x, y)| x + y)
                        .min()
                        .unwrap()
                })
                .min()
                .unwrap()
        })
        .for_each(|ans| println!("{ans}"));
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

use proconio::source::{Readable, Source};
enum Digits {}
impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}
