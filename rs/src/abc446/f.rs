fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let adjs = uv.iter().fold(vec![vec![]; n], |mut adjs, &(u, v)| {
        adjs[u].push(v);
        adjs
    });

    let mut visited = vec![false; n];
    visited[0] = true;
    let mut c = 0;
    let mut d = 1;
    let mut st = vec![];
    for k in 0..n {
        if visited[k] {
            c += 1;

            st.push(k);

            while let Some(z) = st.pop() {
                for &x in &adjs[z] {
                    if !visited[x] {
                        visited[x] = true;
                        d += 1;
                        if x <= k {
                            c += 1;
                            st.push(x);
                        }
                    }
                }
            }
        }

        if c == k + 1 {
            println!("{}", d - c);
        } else {
            println!("-1");
        }
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
