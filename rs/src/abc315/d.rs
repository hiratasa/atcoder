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

#[allow(unused_macros)]
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

trait IterCopyExt<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

impl<'a, T, I> IterCopyExt<'a, T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);
    let c = read_vec(h, || read_str());

    let mut freq_row = c
        .iter()
        .map(|row| {
            row.citer().fold([0; 26], |mut freq, x| {
                freq[x as usize - 'a' as usize] += 1;
                freq
            })
        })
        .collect::<Vec<_>>();

    let mut freq_col = (0..w)
        .map(|j| {
            c.iter().map(|row| row[j]).fold([0; 26], |mut freq, x| {
                freq[x as usize - 'a' as usize] += 1;
                freq
            })
        })
        .collect::<Vec<_>>();

    let mut row_valid = vec![true; h];
    let mut col_valid = vec![true; w];

    let mut num_row = h;
    let mut num_col = w;

    while num_row > 0 && num_col > 0 {
        let row_removed = (0..h)
            .filter(|&i| row_valid[i])
            .filter(|&i| num_col >= 2 && freq_row[i].citer().filter(|&x| x > 0).count() == 1)
            .collect::<Vec<_>>();
        let col_removed = (0..w)
            .filter(|&i| col_valid[i])
            .filter(|&i| num_row >= 2 && freq_col[i].citer().filter(|&x| x > 0).count() == 1)
            .collect::<Vec<_>>();

        if row_removed.is_empty() && col_removed.is_empty() {
            break;
        }

        row_removed.citer().for_each(|i| {
            row_valid[i] = false;
        });
        col_removed.citer().for_each(|i| {
            col_valid[i] = false;
        });

        let num_removed_row_ch = row_removed
            .citer()
            .map(|i| freq_row[i].citer().position(|x| x > 0).unwrap())
            .fold([0; 26], |mut removed_ch, ch| {
                removed_ch[ch] += 1;
                removed_ch
            });
        let num_removed_col_ch = col_removed
            .citer()
            .map(|i| freq_col[i].citer().position(|x| x > 0).unwrap())
            .fold([0; 26], |mut removed_ch, ch| {
                removed_ch[ch] += 1;
                removed_ch
            });

        (0..h).filter(|&i| row_valid[i]).for_each(|i| {
            for ch in 0..26 {
                freq_row[i][ch] -= num_removed_col_ch[ch];
            }
        });
        (0..w).filter(|&i| col_valid[i]).for_each(|i| {
            for ch in 0..26 {
                freq_col[i][ch] -= num_removed_row_ch[ch];
            }
        });

        num_row -= row_removed.len();
        num_col -= col_removed.len();
    }

    println!("{}", num_row * num_col);
}
