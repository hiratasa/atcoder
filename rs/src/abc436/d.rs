fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    };

    let warps = iproduct!(0..h, 0..w).fold(vec![vec![]; 26], |mut warps, (i, j)| {
        if s[i][j] != '.' && s[i][j] != '#' {
            let c = s[i][j] as usize - 'a' as usize;
            warps[c].push((i, j));
        }
        warps
    });

    let mut dists = vec![usize::MAX; h * w + 26];
    let mut q = VecDeque::new();

    dists[0] = 0;
    q.push_back((0, 0));

    while let Some((idx, c)) = q.pop_front() {
        if c != dists[idx] {
            continue;
        }

        if idx < h * w {
            let i = idx / w;
            let j = idx % w;

            // walk
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
                .filter(|&(ni, nj)| ni < h && nj < w)
                .filter(|&(ni, nj)| s[ni][nj] != '#')
                .for_each(|(ni, nj)| {
                    if c + 1 < dists[ni * w + nj] {
                        dists[ni * w + nj] = c + 1;
                        q.push_back((ni * w + nj, c + 1));
                    }
                });

            // warp
            if s[i][j] != '.' {
                let ch = s[i][j] as usize - 'a' as usize;
                if c + 1 < dists[h * w + ch] {
                    dists[h * w + ch] = c + 1;
                    q.push_back((h * w + ch, c + 1));
                }
            }
        } else {
            for &(i, j) in &warps[idx - h * w] {
                if c < dists[i * w + j] {
                    dists[i * w + j] = c;
                    q.push_front((i * w + j, c));
                }
            }
        }
    }

    if dists[h * w - 1] != usize::MAX {
        println!("{}", dists[h * w - 1]);
    } else {
        println!("-1");
    }
}

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
