fn main() {
    input! {
        n: usize, q: usize,
        queries: [(usize, Usize1, Usize1); q],
    };

    queries
        .into_iter()
        .scan(
            (
                (0..n).map(|i| vec![i]).collect::<Vec<_>>(),
                (0..n).map(|i| vec![i]).collect::<Vec<_>>(),
                (0..n).collect::<Vec<_>>(),
            ),
            |(groups, top10, idxs), (t, v, u)| {
                if t == 1 {
                    if idxs[v] != idxs[u] {
                        let (v, u) = if groups[idxs[v]].len() < groups[idxs[u]].len() {
                            (v, u)
                        } else {
                            (u, v)
                        };

                        let idx0 = idxs[v];
                        let idx1 = idxs[u];
                        for &x in &groups[idx0] {
                            idxs[x] = idx1;
                        }

                        let g = take(&mut groups[idx0]);
                        groups[idx1].extend(g);

                        let h = take(&mut top10[idx0]);
                        top10[idx1].extend(h);
                        top10[idx1].sort_by_key(|&x| Reverse(x));
                        top10[idx1].truncate(10);
                    }

                    Some(None)
                } else {
                    Some(Some(
                        top10[idxs[v]].get(u).copied().map_or(-1, |x| x as i64 + 1),
                    ))
                }
            },
        )
        .flatten()
        .for_each(|ans| {
            println!("{ans}");
        });
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
