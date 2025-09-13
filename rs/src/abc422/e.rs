fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let mut rng = SmallRng::seed_from_u64(42);

    let ans = (0..100).find_map(|_| {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n - 1);
        let j = if j >= i { j + 1 } else { j };

        let (x1, y1) = xy[i];
        let (x2, y2) = xy[j];

        let a = y2 - y1;
        let b = -x2 + x1;
        let c = x2 * y1 - y2 * x1;

        if xy.iter().filter(|&(x, y)| a * x + b * y + c == 0).count() >= n / 2 + 1 {
            Some((a, b, c))
        } else {
            None
        }
    });

    if let Some(ans) = ans {
        println!("Yes");
        println!("{} {} {}", ans.0, ans.1, ans.2);
    } else {
        println!("No");
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
use rand::{rngs::SmallRng, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
