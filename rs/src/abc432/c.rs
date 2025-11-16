fn main() {
    input! {
        n: usize, x: i64, y: i64,
        mut a: [i64; n],
    };

    a.sort();
    if let Some((b, c, s)) =
        a.iter()
            .copied()
            .tuple_windows()
            .try_fold((0, 0, 0), |(b0, c0, s0), (a0, a1)| {
                let d = a1 - a0;
                // b * x - c * y = 0
                // b - c = d
                // => b * x - (b - d) * y = 0
                // => b = d * y / (y - x)

                if (d * y) % (y - x) != 0 {
                    return None;
                }

                let b = d * y / (y - x);
                let c = b - d;

                Some((b0 + b, c0 + c, s0 + c0 + c))
            })
    {
        if c > a[0] {
            println!("-1");
            return;
        }

        println!("{}", a[0] * (n as i64) - s);
    } else {
        println!("-1");
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
