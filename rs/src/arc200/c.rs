fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            lr: [(usize, usize); n],
        };

        let mut ans = vec![0; n];
        let mut unused = vec![true; n + 1];
        let mut stack = (0..n).rev().collect::<Vec<_>>();
        while let Some(i) = stack.pop() {
            if ans[i] != 0 {
                continue;
            }
            let t = (0..n)
                .rev()
                .filter(|&j| lr[i].0 < lr[j].0 && lr[j].1 < lr[i].1)
                .filter(|&j| ans[j] == 0)
                .collect::<Vec<_>>();
            let x = (1..=n).filter(|&x| unused[x]).nth(t.len()).unwrap();
            ans[i] = x;
            unused[x] = false;
            stack.extend(t);
        }

        println!("{}", ans.iter().join(" "));
    }
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
