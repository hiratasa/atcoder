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

fn solve(poly: &[[[char; 4]; 4]], idx: usize, filled: u16) -> bool {
    if idx == poly.len() {
        return filled == !0;
    }

    let mut p = poly[idx];
    for _ in 0..4 {
        p = [
            [p[3][0], p[2][0], p[1][0], p[0][0]],
            [p[3][1], p[2][1], p[1][1], p[0][1]],
            [p[3][2], p[2][2], p[1][2], p[0][2]],
            [p[3][3], p[2][3], p[1][3], p[0][3]],
        ];

        let min_i = iproduct!(0..4, 0..4)
            .filter(|&(i, j)| p[i][j] == '#')
            .map(|(i, _)| i)
            .min()
            .unwrap();
        let max_i = iproduct!(0..4, 0..4)
            .filter(|&(i, j)| p[i][j] == '#')
            .map(|(i, _)| i)
            .max()
            .unwrap();
        let min_j = iproduct!(0..4, 0..4)
            .filter(|&(i, j)| p[i][j] == '#')
            .map(|(_, j)| j)
            .min()
            .unwrap();
        let max_j = iproduct!(0..4, 0..4)
            .filter(|&(i, j)| p[i][j] == '#')
            .map(|(_, j)| j)
            .max()
            .unwrap();

        let mask = iproduct!(0..4, 0..4)
            .map(|(i, j)| (p[i][j] == '#') as u16)
            .fold(0, |x, y| 2 * x + y);

        let ok = chain(
            (0..=min_i).map(|s| mask << (4 * s)),
            (0..=3 - max_i).map(|s| mask >> (4 * s)),
        )
        .flat_map(|mask2| {
            chain(
                (0..=min_j).map(move |s| mask2 << s),
                (0..=3 - max_j).map(move |s| mask2 >> s),
            )
        })
        .filter(|&mask2| filled & mask2 == 0)
        .any(|mask2| solve(poly, idx + 1, filled | mask2));

        if ok {
            return true;
        }
    }

    false
}

fn main() {
    input! {
        p: [[Chars; 4]; 3]
    };

    let poly = p
        .iter()
        .map(|pp| {
            let mut q = [['.'; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    q[i][j] = pp[i][j];
                }
            }

            q
        })
        .collect::<Vec<_>>();

    if solve(&poly, 0, 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
