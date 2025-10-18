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

fn solve(
    n: usize,
    r: &[char],
    c: &[char],
    i: usize,
    grid: &mut Vec<Vec<Option<char>>>,
    used: &mut [usize; 3],
    c_not_exists: usize,
) -> bool {
    if i == n {
        let s = (1 << n) - 1;
        return used[0] == s && used[1] == s && used[2] == s;
    }

    iproduct!(0..n, 0..n, 0..n)
        .filter(|&(a, b, c)| a != b && b != c && c != a)
        .map(|(a, b, c)| [a, b, c])
        .any(|abc| {
            if (0..3).any(|i| used[i] & (1 << abc[i]) != 0) {
                return false;
            }

            if (0..3).any(|x| {
                let ch = (b'A' + x as u8) as char;
                let pos = abc[x];

                c_not_exists & (1 << pos) > 0 && ch != c[pos]
            }) {
                return false;
            }

            if abc
                .citer()
                .enumerate()
                .min_by_key(|&(_, pos)| pos)
                .map(|(x, _)| (b'A' + x as u8) as char)
                != Some(r[i])
            {
                return false;
            }

            let mut c_not_exists = c_not_exists;
            (0..3).for_each(|x| {
                let ch = (b'A' + x as u8) as char;
                let pos = abc[x];

                used[x] |= 1 << pos;
                grid[i][pos] = Some(ch);
                c_not_exists &= !(1 << pos);
            });

            if solve(n, r, c, i + 1, grid, used, c_not_exists) {
                return true;
            }

            (0..3).for_each(|x| {
                let pos = abc[x];

                used[x] ^= 1 << pos;
                grid[i][pos] = None;
            });

            false
        })
}

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    };

    let mut grid = vec![vec![None; n]; n];
    if solve(n, &r, &c, 0, &mut grid, &mut [0; 3], (1 << n) - 1) {
        println!("Yes");
        for row in grid {
            println!("{}", row.citer().map(|x| x.unwrap_or('.')).join(""));
        }
    } else {
        println!("No");
    }
}
