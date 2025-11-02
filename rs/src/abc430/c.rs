fn main() {
    input! {
        n: usize, a: usize, b: usize,
        s: Chars,
    };

    let t = once(0)
        .chain(s.iter().copied().map(|c| (c == 'a') as usize))
        .cumsum::<usize>()
        .collect::<Vec<_>>();
    let ps = t
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i - x))
        .collect::<Vec<_>>();

    let ans = ps
        .iter()
        .scan((0, VecDeque::new()), |(j, q), (x, y)| {
            while x - ps[*j].0 >= a {
                q.push_back(ps[*j]);
                *j += 1;
            }

            while let Some(&(x2, y2)) = q.front()
                && y - y2 >= b
            {
                q.pop_front();
            }

            Some(q.len())
        })
        .sum::<usize>();

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
