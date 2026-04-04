fn main() {
    input! {
        t: usize,
        cases: [(usize, Chars, Chars); t],
    };

    // let mut rng = SmallRng::seed_from_u64(42);
    // repeat_with(|| {
    //     let n = rng.random_range(2..12);

    //     let x = (0..n)
    //         .map(|_| rng.random::<bool>())
    //         .map(|x| if x { 'A' } else { 'B' })
    //         .collect::<Vec<_>>();
    //     let y = (0..n)
    //         .map(|_| rng.random::<bool>())
    //         .map(|x| if x { 'A' } else { 'B' })
    //         .collect::<Vec<_>>();

    //     (n, x, y)
    // })
    cases
        .into_iter()
        .map(|(n, x, y)| {
            let sx = once(0)
                .chain(x.iter().copied().map(|c| (c == 'A') as i64))
                .cumsum::<i64>()
                .collect::<Vec<_>>();
            let sy = once(0)
                .chain(y.iter().copied().map(|c| (c == 'A') as i64))
                .cumsum::<i64>()
                .collect::<Vec<_>>();

            if sx[n] != sy[n] {
                return ((n, x, y, vec![]), None);
            }

            let mut ops = vec![];
            solve(&sx, &sy, 0, 0, n, false, usize::MAX, &mut ops);

            if ops
                .iter()
                .copied()
                .filter(|&(p, _, _)| p == usize::MAX)
                .count()
                > 1
            {
                return ((n, x, y, vec![]), None);
            }

            let children = ops.iter().copied().enumerate().skip(1).fold(
                vec![vec![]; ops.len()],
                |mut children, (i, (p, _, _))| {
                    children[p].push(i);
                    children
                },
            );

            let mut ans = vec![];
            if !ops.is_empty() {
                let mut unresolved = vec![];
                build_ans(&x, 0, &ops, &children, &mut ans, &mut unresolved);
                if !unresolved.is_empty() {
                    return ((n, x, y, vec![]), None);
                }
            }

            ((n, x, y, ops), Some(ans))
        })
        // .inspect(|((n, x, y, ops), ans)| {
        //     let x0 = x;
        //     let mut x = x.clone();
        //     if let Some(ans) = ans.as_ref() {
        //         for &(i, j) in ans {
        //             x[i..=j].reverse();
        //         }
        //         assert_eq!(&x, y, "{n} {x0:?} {y:?}\n{ans:?}\n{x:?}\n{ops:?}");
        //     }
        // })
        .for_each(|(_, ans)| {
            if let Some(ans) = ans {
                println!("Yes");
                println!("{}", ans.len());
                for (i, j) in ans {
                    println!("{} {}", i + 1, j + 1);
                }
            } else {
                println!("No");
            }
        })
}

fn solve(
    sx: &[i64],
    sy: &[i64],
    i0: usize,
    i1: usize,
    len: usize,
    rev: bool,
    parent: usize,
    ops: &mut Vec<(usize, usize, usize)>,
) {
    let sum_x = |i: usize, sublen: usize| {
        if !rev {
            sx[i0 + i + sublen] - sx[i0 + i]
        } else {
            sx[i0 + 1 - i] - sx[i0 + 1 - i - sublen]
        }
    };
    let sum_y = |i: usize, sublen: usize| sy[i1 + i + sublen] - sy[i1 + i];
    let sum_delta = |i: usize, sublen: usize| sum_x(i, sublen) - sum_y(i, sublen);
    let get_delta = |i: usize| sum_delta(i, 1);

    let fix_pos = |i: usize| {
        if !rev { i0 + i } else { i0 - i }
    };

    if len == 0 {
        return;
    }

    let mut i = 0;
    let mut j = len - 1;

    assert_eq!(sum_delta(0, len), 0);

    while i < j {
        if let Some(ii) = (i..=j).find(|&i| get_delta(i) != 0) {
            i = ii;
        } else {
            break;
        }

        j = (i + 1..=j).rev().find(|&j| get_delta(j) != 0).unwrap();

        if get_delta(i) + get_delta(j) == 0 {
            ops.push((parent, fix_pos(i), fix_pos(j)));
            solve(
                sx,
                sy,
                fix_pos(j - 1),
                i1 + i + 1,
                j - i - 1,
                !rev,
                ops.len() - 1,
                ops,
            );
            return;
        }

        for sublen in 2.. {
            if sum_delta(i, sublen) == 0 {
                ops.push((parent, fix_pos(i), fix_pos(i + sublen - 1)));
                solve(
                    sx,
                    sy,
                    fix_pos(i + sublen - 2),
                    i1 + i + 1,
                    sublen - 2,
                    !rev,
                    ops.len() - 1,
                    ops,
                );
                i += sublen;
                break;
            }
            if sum_delta(j + 1 - sublen, sublen) == 0 {
                ops.push((parent, fix_pos(j + 1 - sublen), fix_pos(j)));
                solve(
                    sx,
                    sy,
                    fix_pos(j - 1),
                    i1 + j + 2 - sublen,
                    sublen - 2,
                    !rev,
                    ops.len() - 1,
                    ops,
                );
                j -= sublen;
                break;
            }
        }
    }
}

fn build_ans(
    x: &[char],
    idx: usize,
    ops: &[(usize, usize, usize)],
    children: &[Vec<usize>],
    ans: &mut Vec<(usize, usize)>,
    unresolved: &mut Vec<usize>,
) {
    let mut u = vec![];
    children[idx]
        .iter()
        .copied()
        .for_each(|c| build_ans(x, c, ops, children, ans, &mut u));

    let (_, i, j) = ops[idx];
    let (i, j) = (min(i, j), max(i, j));
    if x[i] == 'A' && x[j] == 'B' {
        ans.push((i, j));

        for &c in &u {
            let (_, ii, jj) = ops[c];
            let (ii, jj) = (min(ii, jj), max(ii, jj));
            ans.push((i + j - jj, j - (ii - i)));
        }
    } else {
        unresolved.extend(u);
        unresolved.push(idx);
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
