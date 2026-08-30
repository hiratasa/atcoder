fn main() {
    input! {
        n: usize, k: usize,
    };

    solve(n, k, 0, &mut vec![0; n]);
}

fn solve(n: usize, k: usize, i: usize, buf: &mut [usize]) {
    if i == n - 1 {
        assert!(k % (i + 1) == 0);
        buf[i] = k / (i + 1);
        println!("{}", buf.iter().join(" "));
        return;
    }

    if i + 2 == n {
        let Some(j0) = (0..=k / (i + 1)).find(|j| (k - (i + 1) * j) % (i + 2) == 0) else {
            return;
        };

        for idx in 0.. {
            let x = j0 + idx * (i + 2);
            if (i + 1) * x > k {
                break;
            }

            buf[i] = x;
            solve(n, k - (i + 1) * x, i + 1, buf);
        }
    } else {
        for j in 0..=k {
            if (i + 1) * j > k {
                break;
            }
            buf[i] = j;
            solve(n, k - (i + 1) * j, i + 1, buf);
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
