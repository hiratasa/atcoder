fn main() {
    let mut memo = FxHashMap::default();

    let m = 10;
    for k in 2..3 {
        println!("==={k}===");
        // for a0 in 1..10 {
        //     println!("{a0:02}: ");
        //     for a1 in 1..10 {
        //         print!("# {a1:02}: ");
        //         for a2 in 1..10 {
        //             print!("{}", calc(&mut [a0, a1, a2], k, k, &mut memo) as usize);
        //         }
        //         println!();
        //     }
        //     println!();
        // }
        (1..7)
            .combinations_with_replacement(m)
            .filter(|v| !calc(&mut v.clone(), k, k, &mut memo))
            .for_each(|v| {
                println!("{v:?}, {}", v.iter().sum::<usize>());
            })
    }
}

fn calc(
    a: &mut [usize],
    k: usize,
    r: usize,
    memo: &mut FxHashMap<(Vec<usize>, usize, usize), bool>,
) -> bool {
    if let Some(&x) = memo.get(&(a.to_vec(), k, r)) {
        return x;
    }

    let x = if r == 0 {
        !calc(a, k, k, memo)
    } else {
        let n = a.len();

        (0..n).any(|i| {
            let y = a[i];
            (1..=y).any(|c| {
                a[i] -= c;
                let win = calc(a, k, r - 1, memo);
                a[i] += c;

                win
            })
        })
    };

    memo.insert((a.to_vec(), k, r), x);

    x
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
