fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            a: [i64; n],
            b: [i64; n],
        };

        if let Some(ans) = solve(&a, &b) {
            println!("Yes");
            println!("{}", ans.iter().join(" "));
        } else {
            println!("No");
        }
    }
}

fn solve(a: &[i64], b: &[i64]) -> Option<Vec<i64>> {
    let n = a.len();

    let i0 = 0;
    let i1 = (0..n).find(|&i| a[i0] * b[i] != a[i] * b[i0])?;
    let mut ans = vec![0; n];

    if a[i0] * b[i1] - a[i1] * b[i0] > 0 {
        ans[i0] = a[i1] + b[i1];
        ans[i1] = -(a[i0] + b[i0]);
    } else {
        ans[i0] = -(a[i1] + b[i1]);
        ans[i1] = a[i0] + b[i0];
    }

    Some(ans)
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
