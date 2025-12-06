fn main() {
    input! {
        n: usize,
        s: Chars,
        r: [usize; n],
    };

    let i0 = (0..n).find(|&i| s[i] == '.').unwrap();
    let i1 = (0..n).rev().find(|&i| s[i] == '.').unwrap() + 1;

    let n = i1 - i0;
    let s = s[i0..i1].to_vec();
    let r = r[i0..i1].to_vec();

    let m = r.iter().copied().max().unwrap();

    let chunks = (0..n)
        .chunk_by(|&i| s[i])
        .into_iter()
        .map(|(_, it)| {
            it.fold((0, 0), |(ma, k), i| {
                let x = r[i];
                if x > ma {
                    (x, 1)
                } else if x == ma {
                    (ma, k + 1)
                } else {
                    (ma, k)
                }
            })
        })
        .collect::<Vec<_>>();

    // eprintln!("{chunks:?}");

    let ans = (0..chunks.len() - 1)
        .step_by(2)
        .map(|i| {
            let (ma0, k0) = chunks[i];
            let (ma1, k1) = chunks[i + 1];
            let (ma2, k2) = chunks[i + 2];

            if ma0 == m || ma1 == m || ma2 == m {
                k0 * k2
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("{ans}");
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
