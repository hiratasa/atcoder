fn main() {
    input! {
        x: usize,
    };

    if x == 0 {
        println!("A");
        return;
    }

    let n = 25;
    let m = (x + n - 1) / n;
    let k = n * m - x;

    println!(
        "{}",
        repeat_n(['A', 'R'], n - k)
            .flatten()
            .chain(once('C'))
            .chain(repeat_n(['R', 'A'], k).flatten())
            .chain(repeat_n(['R', 'C'], m - 1).flatten())
            .join("")
    );
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
