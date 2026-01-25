fn main() {
    input! {
        n: usize, q: usize,
        xy: [(i64, i64); n],
        ab: [(Usize1, Usize1); q],
    };

    let t = (0..n)
        .sorted_by(|&i, &j| {
            let (x0, y0) = xy[i];
            let (x1, y1) = xy[j];

            ((y0, x0) < (0, 0))
                .cmp(&((y1, x1) < (0, 0)))
                .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
                .reverse()
        })
        .map(|i| vec![i])
        .coalesce(|mut ii, jj| {
            let a = xy[ii[0]];
            let b = xy[jj[0]];
            if a.0.signum() == b.0.signum()
                && a.1.signum() == b.1.signum()
                && a.0 * b.1 == a.1 * b.0
            {
                ii.extend(jj);
                Ok(ii)
            } else {
                Err((ii, jj))
            }
        })
        .collect::<Vec<_>>();

    let l = t.len();
    let idxs = t.iter().enumerate().fold(vec![0; n], |mut idxs, (i, ii)| {
        for &j in ii {
            idxs[j] = i;
        }
        idxs
    });
    let sums = once(0)
        .chain(t.iter().map(|ii| ii.len()).cycle())
        .take(2 * l + 1)
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    ab.iter()
        .map(|&(i, j)| {
            if idxs[i] <= idxs[j] {
                sums[idxs[j] + 1] - sums[idxs[i]]
            } else {
                n - (sums[idxs[i]] - sums[idxs[j] + 1])
            }
        })
        .for_each(|ans| {
            println!("{ans}");
        });
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
