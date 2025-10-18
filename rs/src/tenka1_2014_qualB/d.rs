#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
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
    }
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let mut c = $c;
        c.push($x);
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
    let a = read_col::<usize>(n);
    let (h, w) = read_tuple!(usize, usize);
    let b = read_mat::<usize>(h);

    let valid = b.iter().flatten().fold(vec![false; n], |mut valid, x| {
        valid[*x] = true;
        valid
    });
    let valid_idxs =
        (0..n)
            .filter(|&i| valid[i])
            .enumerate()
            .fold(vec![None; n], |mut idxs, (idx, i)| {
                idxs[i] = Some(idx);
                idxs
            });

    let bounding_box =
        b.iter()
            .enumerate()
            .fold(vec![(usize::MAX, usize::MAX, 0, 0); n], |bb, (i, row)| {
                row.citer().enumerate().fold(bb, |mut bb, (j, idx)| {
                    let idx = valid_idxs[idx].unwrap();

                    bb[idx].0 = min(bb[idx].0, i);
                    bb[idx].1 = min(bb[idx].1, j);
                    bb[idx].2 = max(bb[idx].2, i);
                    bb[idx].3 = max(bb[idx].3, j);
                    bb
                })
            });

    let dependency = (0..n)
        .map(|idx| {
            iproduct!(
                bounding_box[idx].0..=bounding_box[idx].2,
                bounding_box[idx].1..=bounding_box[idx].3
            )
            .map(|(i, j)| valid_idxs[b[i][j]].unwrap())
            .filter(|&idx2| idx2 != idx)
            .fold(0, |x, idx2| x | (1 << idx2))
        })
        .collect::<Vec<_>>();

    let dp = a
        .citer()
        .sorted()
        .rev()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, it)| it.count())
        .scan(0, |s, k| Some((replace(s, *s + k), k)))
        .fold(vvec![true; false; 1<<n], |dp, (m, k)| {
            successors(Some((1usize << m) - 1), |&s| {
                if s == 0 {
                    None
                } else {
                    let z = s.trailing_zeros();
                    let t = s + (1 << z);
                    let y = (s & !t).count_ones() - 1;

                    Some(t + (1 << y) - 1)
                }
            })
            .take_while(|&s| s < (1 << n))
            .fold(dp, |dp, s| {
                if dp[s] {
                    let s2 = (0..n)
                        .filter(|&i| s & (1 << i) == 0 && (s & dependency[i] == dependency[i]))
                        .fold(0usize, |x, idx| x | (1 << idx));

                    successors(Some(s2), |&t| t.checked_sub(1).map(|t| t & s2))
                        .filter(|&t| t.count_ones() as usize == k)
                        .fold(dp, |mut dp, t| {
                            dp[s | t] = true;

                            dp
                        })
                } else {
                    dp
                }
            })
        });

    if dp[(1 << n) - 1] {
        println!("1");
    } else {
        println!("0");
    }
}
