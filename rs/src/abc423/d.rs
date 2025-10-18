fn main() {
    input! {
        n: usize, k: usize,
        abc: [(usize, usize, usize); n],
    };

    abc.iter()
        .scan(
            (BinaryHeap::new(), 0usize, 0usize),
            |(q, s, t), &(a, b, c)| {
                *t = max(*t, a);
                while *s + c > k {
                    let (Reverse(t1), x) = q.pop().unwrap();
                    *s -= x;
                    *t = max(*t, t1);
                }

                q.push((Reverse(*t + b), c));
                *s += c;

                Some(*t)
            },
        )
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
