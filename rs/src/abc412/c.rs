fn main() {
    input! {
        t: usize,
    };

    (0..t)
        .map(|_| {
            input! {
                s: [usize],
            };

            let src = s[0];
            let dest = s[s.len() - 1];

            let t = s[..s.len() - 1]
                .iter()
                .copied()
                .sorted()
                .skip_while(|&x| x < src)
                .fold(vec![0, src], |mut t, x| {
                    let len = t.len();
                    if t[len - 2] * 2 >= x {
                        t[len - 1] = x;
                    } else if t[len - 1] * 2 >= x {
                        t.push(x);
                    }

                    t
                });

            t.into_iter()
                .position(|x| 2 * x >= dest)
                .map(|i| i as i64 + 1)
                .unwrap_or(-1)
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
