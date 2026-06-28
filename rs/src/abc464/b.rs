fn main() {
    input! {
        h: usize, w: usize,
        c: [Chars; h],
    };

    let i0 = (0..h)
        .find(|&i| c[i].iter().copied().any(|x| x == '#'))
        .unwrap();
    let i1 = (0..h)
        .rev()
        .find(|&i| c[i].iter().copied().any(|x| x == '#'))
        .unwrap();
    let j0 = (0..w).find(|&j| (0..h).any(|i| c[i][j] == '#')).unwrap();
    let j1 = (0..w)
        .rev()
        .find(|&j| (0..h).any(|i| c[i][j] == '#'))
        .unwrap();

    for i in i0..=i1 {
        println!("{}", c[i][j0..=j1].iter().copied().join(""));
    }
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
