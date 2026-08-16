fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort();

    if a.chunk_by(|x, y| x == y)
        .scan(0, |s, x| {
            let ng = *s < x[0];
            *s += x.len();

            Some(ng)
        })
        .any(|ng| ng)
    {
        println!("No");
        return;
    }

    let t = a
        .chunk_by(|x, y| x == y)
        .map(|s| (s[0], s.len()))
        .collect::<Vec<_>>();
    let mut q = vec![];
    let mut ans = vec![];
    let mut cur = 0;
    while ans.len() < n {
        if cur < t.len() && t[cur].0 == ans.len() {
            q.push(t[cur]);
            cur += 1;
        }

        let l = q.len();
        ans.push(q[l - 1].0);
        q[l - 1].1 -= 1;

        if q[l - 1].1 == 0 {
            q.pop();
        }
    }

    println!("Yes");
    println!("{}", ans.iter().join(" "));
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
