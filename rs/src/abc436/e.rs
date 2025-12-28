fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let cycles = p
        .iter()
        .copied()
        .scan(vec![false; n], |visited, i| {
            if !visited[i] {
                Some(Some(
                    iterate(i, |&j| p[j])
                        .skip(1)
                        .take_while(|&j| j != i)
                        .chain(once(i))
                        .inspect(|&j| visited[j] = true)
                        .collect::<Vec<_>>(),
                ))
            } else {
                Some(None)
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let ans = cycles
        .into_iter()
        .map(|cy| cy.len())
        .map(|l| l * (l - 1) / 2)
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
