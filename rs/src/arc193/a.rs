fn main() {
    input! {
        n: usize,
        w: [usize; n],
        lr: [(usize, usize); n],
        q: usize,
        st: [(Usize1, Usize1); q],
    };

    let l_idxs = (0..n).sorted_by_key(|&i| lr[i].0).collect::<Vec<_>>();
    let mut l_mins = vec![usize::MAX; n + 1];
    for i in (0..n).rev() {
        l_mins[i] = min(w[l_idxs[i]], l_mins[i + 1]);
    }

    let r_idxs = (0..n).sorted_by_key(|&i| lr[i].1).collect::<Vec<_>>();
    let mut r_mins = vec![usize::MAX; n + 1];
    for i in 0..n {
        r_mins[i + 1] = min(w[r_idxs[i]], r_mins[i]);
    }

    st.into_iter()
        .map(|(s, t)| {
            let (l0, r0) = lr[s];
            let (l1, r1) = lr[t];

            if r0 < l1 || r1 < l0 {
                return w[s] + w[t];
            }

            let ans0 = {
                let lmin = min(l0, l1);
                let ridx = r_idxs
                    .binary_search_by(|&idx| lr[idx].1.cmp(&lmin).then(Ordering::Greater))
                    .unwrap_err();

                r_mins[ridx]
            };

            let ans1 = {
                let rmax = max(r0, r1);
                let lidx = l_idxs
                    .binary_search_by(|&idx| lr[idx].0.cmp(&rmax).then(Ordering::Less))
                    .unwrap_err();
                l_mins[lidx]
            };

            let ans2 = {
                let lidx = l_idxs
                    .binary_search_by(|&idx| lr[idx].0.cmp(&r0).then(Ordering::Less))
                    .unwrap_err();
                let ridx = r_idxs
                    .binary_search_by(|&idx| lr[idx].1.cmp(&l1).then(Ordering::Greater))
                    .unwrap_err();

                l_mins[lidx].saturating_add(r_mins[ridx])
            };

            let ans3 = {
                let lidx = l_idxs
                    .binary_search_by(|&idx| lr[idx].0.cmp(&r1).then(Ordering::Less))
                    .unwrap_err();
                let ridx = r_idxs
                    .binary_search_by(|&idx| lr[idx].1.cmp(&l0).then(Ordering::Greater))
                    .unwrap_err();

                l_mins[lidx].saturating_add(r_mins[ridx])
            };

            min(min(ans0, ans1), min(ans2, ans3)).saturating_add(w[s] + w[t])
        })
        .for_each(|ans| {
            if ans == usize::MAX {
                println!("-1");
            } else {
                println!("{ans}");
            }
        });
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
