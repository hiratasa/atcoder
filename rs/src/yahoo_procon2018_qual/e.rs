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
    let n = read::<usize>();
    let mut d = read_col::<usize>(n);

    d.sort_by_key(|&x| Reverse(x));

    let sums = once(0)
        .chain(d.citer())
        .cumsum::<usize>()
        .collect::<Vec<_>>();
    let s = sums[n];

    if s % 2 == 0 {
        let ans = (1..=n)
            .scan(n, |j, i| {
                while *j > 0 && d[*j - 1] < i {
                    *j -= 1;
                }

                Some((i, max(i, *j)))
            })
            .all(|(i, j)| sums[i] <= i * (i - 1) + (j - i) * i + sums[n] - sums[j]);

        if ans {
            println!("YES");
        } else {
            println!("ABSOLUTELY NO");
        }
    } else {
        let diffs = once(0)
            .chain(
                (1..=n)
                    .scan(n, |j, i| {
                        while *j > 0 && d[*j - 1] < i {
                            *j -= 1;
                        }

                        Some((i, max(i, *j)))
                    })
                    .map(|(i, j)| {
                        (i * (i - 1) + (j - i) * i + sums[n] - sums[j]) as i64 - sums[i] as i64
                    }),
            )
            .collect::<Vec<_>>();

        let prefix_oks = diffs
            .citer()
            .map(|d| d >= 0)
            .scan(true, |ok0, ok| {
                *ok0 &= ok;
                Some(*ok0)
            })
            .collect::<Vec<_>>();

        let suffix_mins = diffs
            .citer()
            .rev()
            .scan(i64::MAX, |mi, d| {
                *mi = min(*mi, d);
                Some(*mi)
            })
            .collect::<Vec<_>>();

        let ans = (0..n)
            .filter(|&i| i == 0 || d[i - 1] > d[i])
            .map(|i| (i, min(d[i] + 1, i + 1), i + 1))
            .scan((n, n, i64::MAX), |(prev_l, prev_r, mi), (i, l, r)| {
                if l < r {
                    if *prev_l == *prev_r {
                        *prev_l = l;
                        *prev_r = l;
                    }
                    assert!(l <= *prev_l);
                    assert!(r >= *prev_r);

                    for j in l..*prev_l {
                        *mi = min(*mi, diffs[j]);
                    }
                    for j in *prev_r..r {
                        *mi = min(*mi, diffs[j]);
                    }
                    *prev_l = l;
                    *prev_r = r;
                }

                Some((i, l, r, *mi))
            })
            .any(|(i, l, r, mi)| prefix_oks[l - 1] && mi >= -1 && suffix_mins[n - r] >= 1);

        if ans {
            println!("NO");
        } else {
            println!("ABSOLUTELY NO");
        }
    }
}
