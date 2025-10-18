#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
        n: usize,
        mut ab: [(i64, i64); n],
        mut cd: [(i64, i64); n],
    }

    if n == 1 {
        println!("Yes");
        return;
    }

    let g0 = ab.citer().fold((0, 0), |(x, y), (x1, y1)| (x + x1, y + y1));
    let g1 = cd.citer().fold((0, 0), |(x, y), (x1, y1)| (x + x1, y + y1));

    ab.iter_mut().for_each(|(x, y)| {
        *x = *x * n as i64 - g0.0;
        *y = *y * n as i64 - g0.1;
    });
    cd.iter_mut().for_each(|(x, y)| {
        *x = *x * n as i64 - g1.0;
        *y = *y * n as i64 - g1.1;
    });

    let (x0, y0) = ab.citer().find(|&(x, y)| (x, y) != (0, 0)).unwrap();
    let r2 = x0 * x0 + y0 * y0;
    let set1 = cd.citer().collect::<FxHashSet<_>>();

    let ans = cd.citer().any(|(x1, y1)| {
        if r2 != x1 * x1 + y1 * y1 {
            return false;
        }

        let c = x0 * x1 + y0 * y1;
        let s = x0 * y1 - x1 * y0;

        if let Some(set) = ab
            .citer()
            .map(|(x, y)| (c * x - s * y, s * x + c * y))
            .map(|(x, y)| {
                if x % r2 != 0 || y % r2 != 0 {
                    None
                } else {
                    Some((x / r2, y / r2))
                }
            })
            .collect::<Option<FxHashSet<_>>>()
        {
            set == set1
        } else {
            false
        }
    });

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
