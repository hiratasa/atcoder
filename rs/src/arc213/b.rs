fn main() {
    input! {
        cases: [(usize, usize, usize)],
    };

    cases
        .into_iter()
        // use rand::{Rng, SeedableRng, rngs::SmallRng};
        // let rng = &mut SmallRng::seed_from_u64(42);
        // repeat_with(|| {
        //     let q = rng.random_range(0..2);
        //     let l = rng.random_range(0..=100);
        //     let r = rng.random_range(l..=l + 100);
        //     (q, l, r)
        // })
        .for_each(|(q, l, r)| {
            if q == 0 {
                let ans = count(l, r);
                // assert_eq!(
                //     calc(l, r).iter().filter(|&&x| x).count(),
                //     count(l, r),
                //     "{q} {l} {r}"
                // );
                println!("{ans}");
            } else {
                let ans = calc(l, r);
                // assert_eq!(
                //     ans.iter().filter(|&&x| x).count(),
                //     count(l, r),
                //     "{q} {l} {r}"
                // );
                println!("{}", ans.into_iter().map_into::<usize>().join(""));
            }
        });
}

fn count(l: usize, r: usize) -> usize {
    let pattern = detect_pattern(l, r);

    match pattern {
        Ok(x) => {
            let l1 = l ^ ((l ^ x).count_ones() as usize % 2);
            r / 2 - l / 2 + (l <= l1 && l1 <= r) as usize
        }
        Err(a) => r / 2 - l / 2 + 1,
    }
}

fn calc(l: usize, r: usize) -> Vec<bool> {
    let pattern = detect_pattern(l, r);

    match pattern {
        Ok(x) => (l..=r)
            .map(|y| (x ^ y).count_ones() % 2 == 0)
            .collect::<Vec<_>>(),
        Err(a) => (l..=r)
            .map(|y| {
                let x = if y <= a { l } else { r };
                (x ^ y).count_ones() % 2 == 0
            })
            .collect::<Vec<_>>(),
    }
}

fn detect_pattern(l: usize, r: usize) -> Result<usize, usize> {
    match (l % 2, r % 2) {
        (0, 0) => Ok(r),
        (0, 1) => Ok(r),
        (1, 0) => {
            let z = l ^ r;
            let a = (1usize << z.checked_ilog2().unwrap()) - 1;

            let ll = l & a;
            let rr = r & a;

            if ll <= rr { Ok(r) } else { Err((l & !a) ^ a) }
        }
        (1, 1) => Ok(l),
        _ => unreachable!(),
    }
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
