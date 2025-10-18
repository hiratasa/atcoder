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
        h: usize, w: usize,
        a: [Chars; h]
    }

    let (n4, n2, n1) = match (h % 2, w % 2) {
        (0, 0) => (h * w / 4, 0, 0usize),
        (0, 1) => (h * (w - 1) / 4, h / 2, 0),
        (1, 0) => ((h - 1) * w / 4, w / 2, 0),
        (1, 1) => ((h - 1) * (w - 1) / 4, (h - 1 + w - 1) / 2, 1),
        _ => unreachable!(),
    };

    let freq = a.iter().flatten().fold(vec![0; 26], |mut freq, c| {
        freq[*c as usize - b'a' as usize] += 1;
        freq
    });

    let ok = freq
        .citer()
        .try_fold((n4, n2, n1), |(n4, n2, n1), m| {
            let (n4, m) = (n4.saturating_sub(m / 4), m - 4 * min(n4, m / 4));
            let (n2, m) = (n2.saturating_sub(m / 2), m - 2 * min(n2, m / 2));
            let (n1, m) = (n1.saturating_sub(m), m - min(n1, m));

            if m == 0 { Some((n4, n2, n1)) } else { None }
        })
        .is_some();

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
