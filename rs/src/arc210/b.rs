fn main() {
    input! {
        n: usize, m: usize, q: usize,
        a: [usize; n],
        b: [usize; m],
        tix: [(usize, Usize1, usize); q],
    };

    // let nums = a.iter().chain(b.iter()).copied().chain(tix.iter().map(|&(_, _, x)| x)).sorted().dedup().collect::<Vec<_>>();
    // let idxs = nums.iter().enumerate().map(|(i, x)| (x, i)).collect::<FxHashMap<_, _>>();

    let mut c = a
        .iter()
        .chain(b.iter())
        .copied()
        .enumerate()
        .map(|(i, x)| (x, i + 1))
        .collect::<Vec<_>>();
    let mut set = c.iter().copied().collect::<BTreeSet<_>>();
    set.insert((0, 0));
    set.insert((usize::MAX, usize::MAX));

    let k = n / 2;

    let next = |set: &BTreeSet<(usize, usize)>, (x, i): (usize, usize)| {
        set.range((x, i + 1)..).next().copied()
    };
    let prev = |set: &BTreeSet<(usize, usize)>, (x, i): (usize, usize)| {
        set.range(..(x, i)).next_back().copied()
    };

    let mut l = *set.first().unwrap();
    let mut lsum = 0;
    for _ in 0..k {
        l = next(&set, l).unwrap();
        lsum += l.0;
    }

    let mut r = *set.last().unwrap();
    let mut rsum = 0;
    for _ in 0..k {
        r = prev(&set, r).unwrap();
        rsum += r.0;
    }

    let mut idx = n + m + 1;
    let mut lnum = k;
    let mut rnum = k;
    for (t, i, x) in tix {
        let i = if t == 1 { i } else { n + i };

        let old = c[i];
        set.remove(&old);
        if old <= l {
            lsum -= old.0;
            if old == l {
                l = prev(&set, l).unwrap();
            }
            lnum -= 1;
        } else if old >= r {
            rsum -= old.0;
            if old == r {
                r = next(&set, r).unwrap()
            }
            rnum -= 1;
        }

        c[i] = (x, idx);
        idx += 1;
        let new = c[i];
        set.insert(new);
        if new < l {
            lsum += new.0;
            lnum += 1;
        } else if new > r {
            rsum += new.0;
            rnum += 1;
        }

        if lnum < k {
            l = next(&set, l).unwrap();
            lsum += l.0;
            lnum += 1;
        } else if lnum > k {
            lsum -= l.0;
            lnum -= 1;
            l = prev(&set, l).unwrap();
        }
        if rnum < k {
            r = prev(&set, r).unwrap();
            rsum += r.0;
            rnum += 1;
        } else if rnum > k {
            rsum -= r.0;
            rnum -= 1;
            r = next(&set, r).unwrap();
        }

        assert_eq!(lnum, k);
        assert_eq!(rnum, k);

        println!("{}", lsum + rsum);
    }
}

use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
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
use rustc_hash::{FxHashMap, FxHashSet};
