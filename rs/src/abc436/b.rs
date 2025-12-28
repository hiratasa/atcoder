fn main() {
    input! {
        n: usize,
    };

    let mut cells = vec![vec![0; n]; n];
    let mut r = 0;
    let mut c = (n - 1) / 2;
    for i in 1..=n * n {
        cells[r][c] = i;
        let r2 = (r + n - 1) % n;
        let c2 = (c + 1) % n;
        if cells[r2][c2] == 0 {
            r = r2;
            c = c2;
        } else {
            r = (r + 1) % n;
        }
    }

    for row in cells {
        println!("{}", row.iter().join(" "));
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
