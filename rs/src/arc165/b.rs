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

#[allow(dead_code)]
fn solve0(p: &[usize], k: usize) -> usize {
    let n = p.len();
    (0..=n - k)
        .max_by_key(|&i| {
            let mut q = p.to_vec();
            q[i..i + k].sort();

            (q, Reverse(i))
        })
        .unwrap()
}

fn main() {
    input! {
        n: usize, k: usize,
        p: [usize; n]
    }

    let asc_from = p
        .citer()
        .enumerate()
        .scan((0, 0), |(l, prev), (i, x)| {
            if *prev < x {
                *prev = x;
                Some(*l)
            } else {
                *l = i;
                *prev = x;
                Some(*l)
            }
        })
        .collect::<Vec<_>>();

    let asc = p[..n - k]
        .citer()
        .rev()
        .scan((true, usize::MAX), |(asc, prev), x| {
            if *asc && x < *prev {
                *prev = x;
                Some(true)
            } else {
                *asc = false;
                Some(false)
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    let mi = p[n - k..]
        .citer()
        .scan(usize::MAX, |mi, x| {
            *mi = min(*mi, x);
            Some(*mi)
        })
        .collect::<Vec<_>>();

    let i = if let Some(i) = (0..n).find(|&i| i - asc_from[i] + 1 >= k) {
        asc_from[i]
    } else {
        (n.saturating_sub(2 * k - 1)..n - k)
            .find(|&i| asc[i] && p[n - k - 1] < mi[i + k - (n - k) - 1])
            .unwrap_or(n - k)
    };

    let mut ans = p.clone();
    ans[i..i + k].sort();

    println!("{}", ans.citer().join(" "));
}
