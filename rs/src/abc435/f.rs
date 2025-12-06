fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let (mut children, mut t) = p.iter().copied().enumerate().fold(
        (vec![vec![]; n], vec![]),
        |(mut children, mut t): (Vec<Vec<usize>>, Vec<usize>), (i, x)| {
            let mut last = None;
            while let Some(&j) = t.last()
                && p[j] < x
            {
                t.pop().unwrap();
                if let Some(last) = last {
                    children[j].push(last);
                }
                last = Some(j);
            }
            if let Some(last) = last {
                children[i].push(last);
            }
            t.push(i);

            (children, t)
        },
    );
    let root = t[0];
    for (i, j) in t.iter().copied().tuple_windows() {
        children[i].push(j);
    }

    let ans = find_farthest(&children, root);

    println!("{ans}");
}

fn find_farthest(children: &[Vec<usize>], v: usize) -> usize {
    children[v]
        .iter()
        .map(|&u| find_farthest(children, u) + u.abs_diff(v))
        .max()
        .unwrap_or(0)
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
