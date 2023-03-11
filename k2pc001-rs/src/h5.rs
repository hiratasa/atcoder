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
    let (n, q, p) = read_tuple!(usize, usize, usize);
    let a = read_row::<usize>();
    let st = read_vec(q, || read_tuple!(usize, usize));

    let b = max(1, (n as f64 / (q as f64).sqrt()).round() as usize);

    let add = |nums: &mut FxHashMap<usize, usize>,
               scores: &mut FxHashMap<(usize, usize), usize>,
               hash: usize,
               x: usize| {
        let c = nums.entry(x).or_insert(0);
        *c += 1;

        let v = if let Some(&v) = scores.get(&(x, *c)) {
            v
        } else if *c == 1 {
            let v = x % p;
            scores.insert((x, 1), v);
            v
        } else {
            let prev = scores[&(x, *c - 1)];
            let v = prev * x % p;
            scores.insert((x, *c), v);
            v
        };

        (hash + v) % p
    };

    let remove = |nums: &mut FxHashMap<usize, usize>,
                  scores: &mut FxHashMap<(usize, usize), usize>,
                  hash: usize,
                  x: usize| {
        let c = nums.get_mut(&x).unwrap();
        *c -= 1;

        let v = scores[&(x, *c + 1)];

        (hash + p - v) % p
    };

    let ans = st
        .citer()
        .enumerate()
        .sorted_by_key(|&(_, (s, t))| (s / b, t))
        .scan(
            (FxHashMap::default(), FxHashMap::default(), 0, (0, 0)),
            |(nums, scores, hash, (l, r)), (i, (s, t))| {
                let s = s - 1;

                while s < *l {
                    *l -= 1;
                    *hash = add(nums, scores, *hash, a[*l]);
                }

                while *r < t {
                    *hash = add(nums, scores, *hash, a[*r]);
                    *r += 1;
                }

                while *l < s {
                    *hash = remove(nums, scores, *hash, a[*l]);
                    *l += 1;
                }

                while t < *r {
                    *r -= 1;
                    *hash = remove(nums, scores, *hash, a[*r]);
                }

                Some((i, *hash))
            },
        )
        .fold(vec![0; q], |mut ans, (i, x)| {
            ans[i] = x;
            ans
        });

    for x in ans {
        println!("{}", x);
    }
}
