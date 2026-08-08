fn main() {
    input! {
        n: usize, q: usize,
        mut p: [Usize1; n],
    };

    let mut r = p
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; n], |mut r, (i, j)| {
            r[j] = i;
            r
        });

    for _ in 0..q {
        input! {
            ty: usize,
        };

        if ty == 1 {
            input! {
                x: Usize1, y: Usize1,
            };

            r.swap(p[x], p[y]);
            p.swap(x, y);
        } else {
            swap(&mut p, &mut r);
        }
    }

    println!("{}", p.iter().map(|&x| x + 1).join(" "));
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
