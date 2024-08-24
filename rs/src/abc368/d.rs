fn main() {
    input! {
        n: usize, k: usize,
        ab: [(Usize1, Usize1); n - 1],
        v: [Usize1; k],
    };

    let adjs = ab.into_iter().fold(vec![vec![]; n], |mut adjs, (a, b)| {
        adjs[a].push(b);
        adjs[b].push(a);
        adjs
    });

    let mut degs = adjs.iter().map(|a| a.len()).collect::<Vec<_>>();
    let mut ones = degs.iter().positions(|&d| d == 1).collect::<Vec<_>>();

    let set = v.into_iter().collect::<FxHashSet<_>>();

    let mut ans = n;
    while let Some(u) = ones.pop() {
        if set.contains(&u) {
            continue;
        }

        ans -= 1;
        degs[u] -= 1;

        adjs[u].iter().for_each(|&x| {
            if degs[x] > 0 {
                degs[x] -= 1;
                if degs[x] == 1 {
                    ones.push(x);
                }
            }
        });
    }

    println!("{ans}");
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
