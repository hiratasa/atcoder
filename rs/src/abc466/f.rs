fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, x: usize,
            a: [usize; n],
        };

        let b = a.iter().copied().fold(vec![], |mut t, x| {
            if t.last().copied().unwrap_or(usize::MAX) > x {
                t.push(x);
            }

            t
        });

        let ans = calc(&b, 0, x, &mut FxHashMap::default());

        println!("{ans}");
    }
}

fn calc(b: &[usize], i: usize, j: usize, memo: &mut FxHashMap<(usize, usize), usize>) -> usize {
    if j == 0 {
        return 0;
    }

    if i == b.len() {
        return 0;
    }

    if let Some(&x) = memo.get(&(i, j)) {
        return x;
    }

    let idx = i + b[i..]
        .binary_search_by(|&z| z.cmp(&j).then(Ordering::Less).reverse())
        .unwrap_err();
    if idx == b.len() {
        return 0;
    }
    let z = b[idx];

    let q = j / z;
    let r = j % z;

    let x = if q > 0 {
        calc(b, idx + 1, z - 1, memo)
    } else {
        0
    };
    let y = calc(b, idx + 1, r, memo);

    let a = (x + 1) * q + y;

    memo.insert((i, j), a);

    a
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
