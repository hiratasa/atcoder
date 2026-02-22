fn main() {
    input! {
        m: usize, a: usize, b: usize,
    };

    let id = |x: usize, y: usize| x * m + y;

    let g = iproduct!(0..m, 0..m).fold(vec![vec![]; m * m], |mut g, (x, y)| {
        let z = (a * y + b * x) % m;
        g[id(y, z)].push(id(x, y));
        g
    });

    let mut visited = vec![false; m * m];
    let mut stack = (0..m)
        .flat_map(|i| [id(0, i), id(i, 0)])
        .collect::<Vec<_>>();
    while let Some(x) = stack.pop() {
        if visited[x] {
            continue;
        }
        visited[x] = true;
        for &y in &g[x] {
            stack.push(y);
        }
    }

    let ans = visited.into_iter().filter(|&v| !v).count();
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
