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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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

fn main() {
    input! {
        h: usize, w: usize,
        s: [[u8; w]; h],
        xy: [(usize, usize)]
    }

    let (h, w, s, xy) = if h < w {
        (
            w,
            h,
            (0..w)
                .map(|i| (0..h).map(|j| s[j][i]).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            xy.citer().map(|(x, y)| (y, x)).collect::<Vec<_>>(),
        )
    } else {
        (h, w, s, xy)
    };

    let s = s
        .iter()
        .map(|row| row.citer().rev().fold(0usize, |a, b| a * 2 + b as usize))
        .collect::<Vec<_>>();

    let mask = (1 << w) - 1;

    let broken = xy.citer().fold(vec![0usize; h], |mut broken, (x, y)| {
        broken[y] |= 1 << x;
        broken
    });

    let ans = (0usize..1 << w)
        .filter_map(|bs0| {
            (0..h)
                .try_fold(
                    (bs0.count_ones() as usize, bs0, 0),
                    |(num, bs, prev_bs), i| {
                        if bs & broken[i] > 0 {
                            return None;
                        }

                        let next = (s[i] ^ prev_bs ^ bs ^ (bs << 1) ^ (bs >> 1)) & mask;

                        Some((num + next.count_ones() as usize, next, bs))
                    },
                )
                .filter(|&(_, bs, _)| bs == 0)
                .map(|(num, _, _)| num)
        })
        .min()
        .unwrap();

    println!("{ans}");
}
