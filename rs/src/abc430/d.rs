fn main() {
    input! {
        n: usize,
        x: [usize; n],
    };

    x.into_iter()
        .scan((0, once(0).collect::<BTreeSet<_>>()), |(s, set), x| {
            let get_d = |set: &BTreeSet<usize>, x: usize| {
                let dl = set.range(..x).next_back().map_or(usize::MAX, |&l| x - l);
                let dr = set.range(x + 1..).next().map_or(usize::MAX, |&r| r - x);
                min(dl, dr)
            };

            if let Some(&l) = set.range(..x).next_back() {
                let d = get_d(set, l);

                if x - l < d {
                    if d == usize::MAX {
                        *s += x - l;
                    } else {
                        *s = *s - d + x - l;
                    }
                }
            }

            if let Some(&r) = set.range(x + 1..).next() {
                let d = get_d(set, r);

                if r - x < d {
                    if d == usize::MAX {
                        *s += r - x;
                    } else {
                        *s = *s - d + r - x;
                    }
                }
            }

            set.insert(x);
            *s += get_d(set, x);

            Some(*s)
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
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
use rustc_hash::{FxHashMap, FxHashSet};
