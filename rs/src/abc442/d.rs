fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
    };

    (0..q)
        .scan(
            once(0).chain(a).cumsum::<usize>().collect::<Vec<_>>(),
            |b, _| {
                input! {
                    t: usize,
                };

                if t == 1 {
                    input! {
                        x: usize,
                    };

                    let y = b[x] - b[x - 1];
                    let z = b[x + 1] - b[x];

                    b[x] = b[x - 1] + b[x + 1] - b[x];

                    Some(None)
                } else {
                    input! {
                        l: Usize1, r: usize,
                    };

                    Some(Some(b[r] - b[l]))
                }
            },
        )
        .flatten()
        .for_each(|ans| {
            println!("{ans}");
        });
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
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
