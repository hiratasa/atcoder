fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, m: usize,
            xy: [(Usize1, Usize1); m],
            p: [Usize1; n],
        };

        let (adjs, mut in_degs) = xy.iter().fold(
            (vec![vec![]; n], vec![0; n]),
            |(mut adjs, mut in_degs), &(x, y)| {
                adjs[x].push(y);
                in_degs[y] += 1;

                (adjs, in_degs)
            },
        );

        let mut t = (0..n)
            .filter(|&i| in_degs[i] == 0)
            .map(|i| (Reverse(0), i))
            .collect::<BinaryHeap<_>>();

        let mut fixed = vec![false; n];
        let mut lows = vec![0; n];
        for i in 0..n {
            if fixed[i] {
                continue;
            }

            let mut v = vec![];
            while matches!(t.peek(), Some(&(Reverse(l), _)) if l <= i) {
                let (Reverse(l), y) = t.pop().unwrap();
                v.push((l, y));
            }

            assert!(v.len() >= 1);
            v.sort_by_key(|&(_, y)| p[y]);

            for j in 0..v.len() {
                let (_, y) = v[j];
                if j > 0 {
                    fixed[p[y]] = true;
                }

                for &z in &adjs[y] {
                    lows[z] = max(lows[z], p[y] + 1);
                    in_degs[z] -= 1;
                    if in_degs[z] == 0 {
                        t.push((Reverse(lows[z]), z));
                    }
                }
            }
        }

        let ans = fixed.iter().filter(|&&b| b).count();

        println!("{ans}");
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
