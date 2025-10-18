#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::source::{Readable, Source};

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
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let x = $x;
        let mut c = $c;
        c.push(x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
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

fn solve(
    cards: &[f64],
    open: usize,
    phase: usize,
    memo: &mut FxHashMap<(usize, usize), f64>,
) -> f64 {
    if phase == 0 && open.count_ones() <= 1 {
        return 0.0;
    }

    if let Some(&r) = memo.get(&(open, phase)) {
        return r;
    }

    let n = cards.len();
    let r = match phase {
        0 => {
            let m = (0..n).filter(|&i| open & (1 << i) > 0).count();
            (0..n)
                .filter(|&i| open & (1 << i) > 0)
                .map(|i| solve(cards, open ^ (1 << i), 1, memo))
                .sum::<f64>()
                / (m as f64)
        }
        1 => {
            let m = (0..n).filter(|&i| open & (1 << i) > 0).count();
            (0..n)
                .filter(|&i| open & (1 << i) > 0)
                .map(|i| solve(cards, open ^ (1 << i), 2, memo))
                .sum::<f64>()
                / (m as f64)
        }
        2 => {
            let m = (0..n).filter(|&i| open & (1 << i) > 0).count();
            (0..n)
                .filter(|&i| open & (1 << i) == 0)
                .map(|i| solve(cards, open ^ (1 << i), 0, memo) + cards[i])
                .sum::<f64>()
                / ((n - m) as f64)
        }
        _ => unreachable!(),
    };

    memo.insert((open, phase), r);

    r
}

fn main() {
    input! {
        n: usize,
        cards: [f64; n]
    }

    println!(
        "{}",
        solve(&cards, (1 << n) - 1, 0, &mut FxHashMap::default())
    );
}
