fn main() {
    input!{
        n: usize, q: usize,
        lr: [(Usize1, usize); q],
    };

    lr.into_iter().scan((once((0, n)).collect::<BTreeMap<_, _>>(), n), |(map, s), (l, r)| {
        while let Some((&x, &y)) = map.range(l..r).next() {
            map.remove(&x);
            *s -= y - x;

            if y > r {
                map.insert(r, y);
                *s += y - r;
                break;
            }
        }

        if let Some((&x, &y)) = map.range(..l).next_back() {
            if y > l {
                map.remove(&x);
                *s -= y - x;
                map.insert(x, l);
                *s += l - x;
                if y > r {
                    map.insert(r, y);
                    *s += y - r;
                }
            }
        }

        Some(*s)
    }).for_each(|ans| {
        println!("{ans}");
    });
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
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
