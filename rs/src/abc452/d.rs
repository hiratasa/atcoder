fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut nexts = s
        .iter()
        .copied()
        .enumerate()
        .rev()
        .scan(vec![None; 26], |nexts, (i, c)| {
            nexts[c as usize - 'a' as usize] = Some(i);
            Some(nexts.clone())
        })
        .collect::<Vec<_>>();
    nexts.reverse();

    let ans = (0..s.len())
        .map(|i| {
            if let Some((j, _)) = successors(Some((i, 0)), |&(j, k)| {
                if j == s.len() || k == t.len() {
                    None
                } else {
                    nexts[j][t[k] as usize - 'a' as usize].map(|pos| (pos + 1, k + 1))
                }
            })
            .nth(t.len())
            {
                j - i - 1
            } else {
                s.len() - i
            }
        })
        .sum::<usize>();

    println!("{ans}");
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
