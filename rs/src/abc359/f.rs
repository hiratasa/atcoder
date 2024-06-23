fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let ans0 = a.iter().sum::<usize>();

    let ans1 = (0..n - 2)
        .scan(
            (0..n)
                .map(|i| (Reverse(3 * a[i]), i, 2usize))
                .collect::<BinaryHeap<_>>(),
            |q, _| {
                let (Reverse(x), i, t) = q.pop().unwrap();

                q.push((
                    Reverse(((t + 1).pow(2) - t.pow(2)).saturating_mul(a[i])),
                    i,
                    t + 1,
                ));

                Some(x)
            },
        )
        .sum::<usize>();

    let ans = ans0 + ans1;

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
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
