fn main() {
    input! {
        h: usize, w: usize, n: usize,
        hw: [(usize, usize); n],
    };

    let mut q0 = (0..n).map(|i| (hw[i].0, i)).collect::<BinaryHeap<_>>();
    let mut q1 = (0..n).map(|i| (hw[i].1, i)).collect::<BinaryHeap<_>>();

    let mut hh = h;
    let mut ww = w;
    let mut used = vec![false; n];
    let mut ans = vec![(0, 0); n];
    for _ in 0..n {
        if let Some(&(h1, i)) = q0.peek()
            && h1 == hh
        {
            q0.pop();
            let w1 = hw[i].1;
            ans[i] = (0, ww - w1);
            ww -= w1;
            used[i] = true;
        } else if let Some(&(w1, i)) = q1.peek()
            && w1 == ww
        {
            q1.pop();
            let h1 = hw[i].0;
            ans[i] = (hh - h1, 0);
            hh -= h1;
            used[i] = true;
        } else {
            unreachable!();
        }

        while matches!(q0.peek(), Some(&(_, i)) if used[i]) {
            q0.pop();
        }
        while matches!(q1.peek(), Some(&(_, i)) if used[i]) {
            q1.pop();
        }
    }

    for (a, b) in ans {
        println!("{} {}", a + 1, b + 1);
    }
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
