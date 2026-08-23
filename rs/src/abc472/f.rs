fn main() {
    input! {
        n: usize, q: usize,
        xy: [(i128, i128); n],
        uv: [(Usize1, Usize1); q],
    };

    let sx = once(0)
        .chain(
            xy.iter()
                .copied()
                .cycle()
                .tuple_windows()
                .take(2 * n)
                .map(|((x0, y0), (x1, y1))| (x0 + x1) * (x0 * y1 - x1 * y0)),
        )
        .cumsum::<i128>()
        .collect::<Vec<_>>();
    let sy = once(0)
        .chain(
            xy.iter()
                .copied()
                .cycle()
                .tuple_windows()
                .take(2 * n)
                .map(|((x0, y0), (x1, y1))| (y0 + y1) * (x0 * y1 - x1 * y0)),
        )
        .cumsum::<i128>()
        .collect::<Vec<_>>();
    let s = once(0)
        .chain(
            xy.iter()
                .copied()
                .cycle()
                .tuple_windows()
                .take(2 * n)
                .map(|((x0, y0), (x1, y1))| (x0 * y1 - x1 * y0)),
        )
        .cumsum::<i128>()
        .collect::<Vec<_>>();

    uv.iter()
        .copied()
        .map(|(u, v)| {
            let (x0, y0) = xy[u];
            let (x1, y1) = xy[v];

            let v = if v < u { v + n } else { v };

            let z = x1 * y0 - x0 * y1;

            let a = (s[v] - s[u] + z) as f64 / 2.0;

            (
                (sx[v] - sx[u] + (x0 + x1) * z) as f64 / (6.0 * a),
                (sy[v] - sy[u] + (y0 + y1) * z) as f64 / (6.0 * a),
            )
        })
        .for_each(|(x, y)| {
            println!("{x} {y}");
        });
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
