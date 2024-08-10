fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut sets = uv.into_iter().fold(vec![0usize; n], |mut sets, (i, j)| {
        sets[i] |= 1 << j;
        sets[j] |= 1 << i;
        sets
    });

    let mut bits = vec![];
    for i in 0..n {
        let b = bits.len();
        let Some(idx) = (b..n).find(|&idx| sets[idx] & (1<<i) > 0) else { continue };
        sets.swap(b, idx);

        (0..n).filter(|&j| j != b).for_each(|j| {
            if sets[j] & (1 << i) > 0 {
                sets[j] ^= sets[b];
            }
        });

        bits.push(i);
    }

    sets.truncate(bits.len());

    if sets.iter().any(|x| x.count_ones() == 1) {
        println!("No");
        return;
    }

    let k = bits.len();
    let t = (0..n)
        .map(|i| {
            let i1 = if let Some(idx) = bits.iter().position(|&b| b == i) {
                (sets[idx] ^ (1 << i)).trailing_zeros() as usize
            } else {
                i
            };

            (0..k)
                .map(|j| {
                    if sets[j] & (1 << i1) > 0 {
                        1 << bits[j]
                    } else {
                        0
                    }
                })
                .fold(1usize << i1, |x, y| x ^ y)
        })
        .collect::<Vec<_>>();

    let ans = (0..n)
        .map(|i| t.iter().map(|&x| (x >> i) & 1).fold(0, |x, y| (x << 1) | y))
        .collect::<Vec<_>>();

    println!("Yes");

    println!("{}", ans.into_iter().join(" "));
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
