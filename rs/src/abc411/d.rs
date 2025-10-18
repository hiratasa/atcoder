fn main() {
    input! {
        n: usize, q: usize,
    };

    let (buf, idxs) = (0..q).fold(
        (vec![(vec![], None)], vec![0; n + 1]),
        |(mut buf, mut idxs), _| {
            input! {
                ty: usize,
            };

            match ty {
                1 => {
                    input! {
                        p: usize,
                    };

                    idxs[p] = idxs[0];
                }
                2 => {
                    input! {
                        p: usize, s: Chars,
                    };

                    buf.push((s, Some(idxs[p])));
                    idxs[p] = buf.len() - 1;
                }
                3 => {
                    input! {
                        p: usize,
                    };

                    idxs[0] = idxs[p];
                }
                _ => unreachable!(),
            }

            (buf, idxs)
        },
    );

    let ans = successors(Some(idxs[0]), |&i| buf[i].1)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|i| &buf[i].0)
        .flatten()
        .collect::<String>();
    println!("{ans}");
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
