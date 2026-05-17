fn main() {
    input! {
        x: usize,
        q: usize,
        ab: [(usize, usize); q],
    };

    ab.into_iter()
        .scan(
            (BinaryHeap::new(), x, BinaryHeap::new()),
            |(q0, c, q1), (a, b)| {
                if a <= *c {
                    q0.push(a);
                } else {
                    q1.push(Reverse(a));
                }
                if b <= *c {
                    q0.push(b);
                } else {
                    q1.push(Reverse(b));
                }

                while q0.len() > q1.len() {
                    let x = q0.pop().unwrap();
                    let y = min(x, *c);
                    let z = max(x, *c);

                    q1.push(Reverse(z));

                    *c = y;
                }
                while q0.len() < q1.len() {
                    let x = q1.pop().unwrap().0;
                    let y = min(x, *c);
                    let z = max(x, *c);

                    q0.push(y);

                    *c = z;
                }

                Some(*c)
            },
        )
        .for_each(|ans| {
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
