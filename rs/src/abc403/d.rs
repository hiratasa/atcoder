fn main() {
    input! {
        n: usize, d: usize,
        a: [usize; n],
    };

    let mut map = a.into_iter().fold(BTreeMap::new(), |mut map, x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });

    if d == 0 {
        let ans = map.values().map(|x| x - 1).sum::<usize>();
        println!("{ans}");
        return;
    }

    let mut ans = 0;
    while let Some((k, v)) = map.pop_first() {
        let mut t = vec![v];
        for i in 1.. {
            if let Some(v2) = map.remove(&(k + i * d)) {
                t.push(v2);
            } else {
                break;
            }
        }

        let dp = t
            .into_iter()
            .fold([0; 2], |dp, x| [dp[1], min(dp[0], dp[1]) + x]);

        ans += min(dp[0], dp[1]);
    }

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
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
