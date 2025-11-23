fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, m: usize,
            x: [usize; n],
            y: [usize; m],
        };

        let p = (0..n).sorted_by_key(|&i| x[i]).rev().collect::<Vec<_>>();
        let q = (0..m).sorted_by_key(|&i| y[i]).rev().collect::<Vec<_>>();

        if let Some((ans, _, _, _)) = (1..=n * m).rev().try_fold(
            (vec![vec![0; m]; n], 0, 0, vec![]),
            |(mut mat, mut i0, mut j0, mut cells), idx| {
                if i0 < n && idx <= x[p[i0]] {
                    for j in 0..j0 {
                        cells.push((p[i0], q[j]));
                    }
                    i0 += 1;
                }
                if i0 < n && idx <= x[p[i0]] {
                    return None;
                }
                if j0 < m && idx <= y[q[j0]] {
                    for i in 0..i0 {
                        cells.push((p[i], q[j0]));
                    }
                    j0 += 1;
                }
                if j0 < m && idx <= y[q[j0]] {
                    return None;
                }

                if let Some((i, j)) = cells.pop() {
                    mat[i][j] = idx;
                } else {
                    return None;
                }

                Some((mat, i0, j0, cells))
            },
        ) {
            println!("Yes");
            for i in 0..n {
                println!("{}", ans[i].iter().join(" "));
            }
        } else {
            println!("No");
        }
    }
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
