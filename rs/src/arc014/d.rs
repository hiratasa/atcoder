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
        a: usize, n: usize, m: usize,
        l: [usize; n],
        xy: [(usize, usize); m],
    }

    let spaces = l
        .citer()
        .tuple_windows()
        .map(|(s0, s1)| s1 - s0 - 1)
        .sorted()
        .collect::<Vec<_>>();
    let spaces_sum = once(0)
        .chain(spaces.citer())
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    let first_space = l[0] - 1;
    let last_space = a - l[n - 1];

    xy.citer()
        .map(|(x, y)| {
            let z = x + y;

            let idx = spaces
                .binary_search_by(|&s| s.cmp(&z).then(Ordering::Greater))
                .unwrap_err();

            let mut ans = n * (1 + z) - idx * z + spaces_sum[idx];
            if first_space < x {
                ans = ans - x + first_space;
            }
            if last_space < y {
                ans = ans - y + last_space;
            }

            ans
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}
