#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{Readable, Source},
};

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
        bs
    }};
}

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

enum Digits {}

impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}

fn main() {
    input! {
        s: (f64, f64), t: (f64, f64),
        n: usize,
        circles: [((f64, f64), f64); n],
    };

    let mut dists = vec![vec![0.0; n + 2]; n + 2];
    for i in 0..n {
        let ((x0, y0), r0) = circles[i];
        for j in 0..n {
            let ((x1, y1), r1) = circles[j];

            let d = ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt();

            dists[i][j] = f64::max(0.0, d - (r0 + r1));
        }

        for (k, (x1, y1)) in [s, t].citer().enumerate() {
            let d = ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt();
            dists[i][n + k] = f64::max(0.0, d - r0);
            dists[n + k][i] = f64::max(0.0, d - r0);
        }
    }

    {
        let (x0, y0) = s;
        let (x1, y1) = t;

        let d = ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt();
        dists[n][n + 1] = d;
        dists[n + 1][n] = d;
    }

    let mut visited = vec![false; n + 2];
    let mut costs = vec![f64::INFINITY; n + 2];
    costs[n] = 0.0;

    loop {
        let v = (0..n + 2)
            .filter(|&i| !visited[i])
            .min_by_key(|&i| ordered_float::OrderedFloat(costs[i]))
            .unwrap();
        visited[v] = true;
        let cost = costs[v];

        if v == n + 1 {
            println!("{cost}");
            return;
        }

        (0..n + 2).for_each(|i| {
            if cost + dists[v][i] < costs[i] {
                costs[i] = cost + dists[v][i];
            }
        });
    }
}
