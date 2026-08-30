fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    };

    let (ans, _) = once(0)
        .chain(a.iter().copied().cumsum::<usize>())
        .map(|x| x % k)
        .fold((0, FxHashMap::default()), |(m, mut map), x| {
            let z = max(m, map.get(&x).copied().map_or(0, |z| z + 1));
            map.insert(x, z);

            (z, map)
        });

    println!("{ans}");
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
