fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); n],
    };

    let mut points = ab
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (x, y))| (x, y, 1, i))
        .chain(
            cd.iter()
                .copied()
                .enumerate()
                .map(|(i, (x, y))| (x, y, -1, i)),
        )
        .collect::<Vec<_>>();

    let mut ans = calc(&mut points);

    ans.sort();

    println!("{}", ans.into_iter().map(|(_, i)| i + 1).join(" "));
}

fn calc(points: &mut [(i64, i64, i32, usize)]) -> Vec<(usize, usize)> {
    assert!(points.len() % 2 == 0);

    let n = points.len() / 2;

    if n == 0 {
        return vec![];
    }

    points.sort();

    let (x, y, _, _) = points[0];

    points[1..].sort_by(|&(x0, y0, _, _), &(x1, y1, _, _)| {
        ((x0 - x) * (y1 - y) - (x1 - x) * (y0 - y))
            .cmp(&0)
            .reverse()
    });

    let idx = points
        .iter()
        .scan(0, |c, (_, _, d, _)| {
            *c += d;

            Some(*c)
        })
        .position(|c| c == 0)
        .unwrap();

    let mut ans0 = calc(&mut points[1..idx]);
    let ans1 = calc(&mut points[idx + 1..]);

    ans0.extend(ans1);
    if points[0].2 == 1 {
        ans0.push((points[0].3, points[idx].3));
    } else {
        ans0.push((points[idx].3, points[0].3));
    }

    ans0
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
