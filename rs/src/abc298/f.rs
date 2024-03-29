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
    let n = read::<usize>();
    let rcx = read_vec(n, || read_tuple!(usize, usize, usize));

    let row_sums = rcx
        .citer()
        .fold(FxHashMap::default(), |mut sums, (r, _, x)| {
            *sums.entry(r).or_insert(0) += x;
            sums
        });
    let col_sums = rcx
        .citer()
        .fold(FxHashMap::default(), |mut sums, (_, c, x)| {
            *sums.entry(c).or_insert(0) += x;
            sums
        });

    // let ordered_rows = row_sums
    //     .iter()
    //     .map(|(&i, &s)| (s, i))
    //     .sorted_by_key(|&t| Reverse(t))
    //     .collect::<Vec<_>>();
    let ordered_cols = col_sums
        .iter()
        .map(|(&i, &s)| (s, i))
        .sorted_by_key(|&t| Reverse(t))
        .collect::<Vec<_>>();

    let set = rcx
        .citer()
        .map(|(r, c, _)| (r, c))
        .collect::<FxHashSet<_>>();
    let nums_by_rows = rcx
        .citer()
        .fold(FxHashMap::default(), |mut nums_by_rows, (r, _, _)| {
            *nums_by_rows.entry(r).or_insert(0) += 1;

            nums_by_rows
        });

    let ans0 = rcx
        .citer()
        .map(|(r, c, x)| row_sums[&r] + col_sums[&c] - x)
        .max()
        .unwrap();
    let ans1 = row_sums
        .iter()
        .map(|(&r, &s)| {
            ordered_cols
                .citer()
                .take(nums_by_rows[&r] + 1)
                .find(|&(_, c)| !set.contains(&(r, c)))
                .map(|(s1, _)| s + s1)
                .unwrap_or(0)
        })
        .max()
        .unwrap();
    let ans2 = ordered_cols[0].0;

    // dbg!(nums_by_rows);
    // dbg!(row_sums);
    // dbg!(ordered_cols);
    // dbg!(ans0, ans1, ans2);

    println!("{}", max(ans0, max(ans1, ans2)));
}
