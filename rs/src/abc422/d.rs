fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut ans = vec![0; 1 << n];
    solve(0, 1 << n, k, &mut ans);

    let u = if k % (1 << n) == 0 { 0 } else { 1 };

    println!("{u}");
    println!("{}", ans.iter().join(" "));
}

fn solve(l: usize, r: usize, k: usize, ans: &mut [usize]) {
    let n = r - l;

    if n == 1 {
        ans[l] = k;
    } else if k % 2 == 0 {
        solve(l, l + n / 2, k / 2, ans);
        solve(l + n / 2, r, k / 2, ans);
    } else {
        solve(l, l + n / 2, k / 2, ans);
        solve(l + n / 2, r, k / 2 + 1, ans);
    }
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
