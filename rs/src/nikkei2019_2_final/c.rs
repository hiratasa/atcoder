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
    let (h, w, k) = read_tuple!(usize, usize, usize);
    let ab = read_vec(k, || read_tuple!(usize, usize));

    let ab = ab
        .citer()
        .map(|(a, b)| (a - 1, b - 1))
        .collect::<FxHashSet<_>>();

    let top = (0..w).fold(vec![vec![0; w]; h], |mut top, j| {
        (0..h)
            .map(|i| (i, !ab.contains(&(i, j))))
            .scan(0, |l, (i, x)| {
                if x {
                    *l += 1;
                } else {
                    *l = 0;
                }

                Some((i, *l))
            })
            .for_each(|(i, l)| {
                top[i][j] = l;
            });

        top
    });

    let bottom = (0..w).fold(vec![vec![0; w]; h], |mut bottom, j| {
        (0..h)
            .rev()
            .map(|i| (i, !ab.contains(&(i, j))))
            .scan(0, |l, (i, x)| {
                if x {
                    *l += 1;
                } else {
                    *l = 0;
                }

                Some((i, *l))
            })
            .for_each(|(i, l)| {
                bottom[i][j] = l;
            });

        bottom
    });

    let ans = chain((0..w).map(|j| (0, j)), (0..h).map(|i| (i, 0)))
        .map(|(i0, j0)| {
            (0..)
                .map(|idx| (i0 + idx, j0 + idx))
                .take_while(|&(i, j)| i < h && j < w)
                .map(|(i, j)| (i, j, !ab.contains(&(i, j))))
                .scan(BTreeMap::new(), |map, (i, j, x)| {
                    if !x {
                        map.clear();
                        Some(0)
                    } else {
                        map.insert(i, bottom[i][j]);

                        let mut removed = vec![];
                        let mut l1 = 1;
                        for (&ii, &lbottom) in map.range((i + 1).saturating_sub(top[i][j])..) {
                            let c = i - ii + 1;

                            if lbottom < c {
                                removed.push(ii);
                            } else {
                                l1 = c;
                                break;
                            }
                        }

                        for ii in removed {
                            map.remove(&ii);
                        }

                        Some(l1)
                    }
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
