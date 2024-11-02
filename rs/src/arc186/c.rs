fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, m: usize,
            mut vp: [(i64, i64); n],
        };

        if m > n {
            println!("0");
            continue;
        }

        vp.sort_by_key(|&(v, _p)| v);
        let sums = once(0)
            .chain(vp.iter().map(|&(v, p)| (v - p).max(0)))
            .cumsum::<i64>()
            .collect::<Vec<_>>();

        let ans = once((n, (1 << 50, 1 << 50)))
            .chain(vp.iter().copied().enumerate().rev())
            .scan((BinaryHeap::new(), 0), |(q, s), (i, (_v, p))| {
                q.push(p);
                *s += p;
                if q.len() >= m {
                    *s -= q.pop().unwrap();
                }

                if q.len() == m - 1 {
                    Some(m as i64 - 1 - *s + sums[i])
                } else {
                    Some(0)
                }
            })
            .chain(once(0))
            .max()
            .unwrap();

        println!("{ans}");
    }
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
