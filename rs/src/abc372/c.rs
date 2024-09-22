fn main() {
    input! {
        n: usize, q: usize,
        s: Chars,
        xc: [(Usize1, char); q],
    };

    let count = s
        .iter()
        .copied()
        .tuple_windows::<(_, _, _)>()
        .filter(|&t| t == ('A', 'B', 'C'))
        .count();

    xc.into_iter()
        .scan((s, count), |(s, count), (x, c)| {
            if x >= 2 && &s[x - 2..=x] == &['A', 'B', 'C'] {
                *count -= 1;
            } else if x >= 1 && x + 1 < n && &s[x - 1..=x + 1] == &['A', 'B', 'C'] {
                *count -= 1;
            } else if x + 2 < n && &s[x..=x + 2] == &['A', 'B', 'C'] {
                *count -= 1;
            }

            s[x] = c;
            if x >= 2 && &s[x - 2..=x] == &['A', 'B', 'C'] {
                *count += 1;
            } else if x >= 1 && x + 1 < n && &s[x - 1..=x + 1] == &['A', 'B', 'C'] {
                *count += 1;
            } else if x + 2 < n && &s[x..=x + 2] == &['A', 'B', 'C'] {
                *count += 1;
            }

            Some(*count)
        })
        .for_each(|ans| {
            println!("{ans}");
        });
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
