fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut t = vec![Ok(vec![]); n];
    let mut path = vec![];
    let mut visiting = vec![usize::MAX; n];
    for i in 0..n {
        dfs(&a, i, 0, &mut path, &mut visiting, &mut t);
    }

    println!(
        "{}",
        (0..n)
            .map(|i| {
                match &t[i] {
                    Ok(path) => {
                        assert_eq!(path[0], i);
                        let r = pow_mod(10, 100, path.len());

                        path[r]
                    }
                    &Err((u, d)) => {
                        let path = t[u].as_ref().unwrap();
                        let r = pow_mod(10, 100, path.len());
                        let r2 = (r + path.len() * d - d) % path.len();

                        path[r2]
                    }
                }
            })
            .map(|x| x + 1)
            .join(" ")
    );
}

fn dfs(
    a: &[usize],
    v: usize,
    idx: usize,
    path: &mut Vec<usize>,
    visiting: &mut [usize],
    t: &mut [Result<Vec<usize>, (usize, usize)>],
) -> (usize, usize) {
    if !matches!(&t[v], Ok(l) if l.is_empty()) {
        // visited
        return match &t[v] {
            Ok(_) => (v, 0),
            &Err((u, d)) => (u, d),
        };
    }

    if visiting[v] < idx {
        let l = idx - visiting[v];
        t[v] = Ok(path[path.len() - l..].to_vec());
        return (v, 0);
    }

    visiting[v] = idx;
    path.push(v);

    let (u, d) = dfs(a, a[v], idx + 1, path, visiting, t);

    path.pop();
    visiting[v] = usize::MAX;

    if matches!(&t[v], Ok(l) if l.is_empty()) {
        t[v] = Err((u, d + 1));
    }

    (u, d + 1)
}

pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;

    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y
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
