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
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, Itertools};
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
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
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
    let b = read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();
    let price = read::<usize>();

    const A: usize = 200;
    const B: usize = 100;
    const C: usize = B * 9 + 1;

    let c = b.citer().fold(
        vvec![(0, vec![]); (usize::MAX, vec![]); C],
        |c: Vec<(usize, Vec<usize>)>, bb| {
            (0..C - bb).fold(c, move |mut c, i| {
                if c[i].0.saturating_add(1) < c[i + bb].0 {
                    c[i + bb] = (c[i].0 + 1, c[i].1.citer().chain(once(bb)).collect_vec());
                }
                c
            })
        },
    );

    let digits = iterate(price, |p| p / 10)
        .take_while(|x| *x > 0)
        .map(|x| x % 10)
        .collect_vec()
        .into_iter()
        .rev()
        .collect_vec();

    let mut init = vec![vec![(usize::MAX, None); B]; B];
    init[0][0] = (0, None);
    let dp = digits.into_iter().fold(vec![init], |dp, d| {
        let prev = dp.last().unwrap();
        let next =
            iproduct!(0..B, 0..B).fold(vec![vec![(usize::MAX, None); B]; B], |next, (i, j)| {
                c.iter()
                    .enumerate()
                    .take_while(|(k, (l, _))| *k <= i * 10 + d)
                    .skip_while(|(k, (l, _))| i * 10 + d >= k + B)
                    .filter(|(k, (l, _))| *l < B)
                    .fold(next, |mut next, (k, (l, _))| {
                        let i1 = i * 10 + d - k;
                        let (l1, x) = if j < *l {
                            (*l, prev[i][j].0.saturating_add(j + (l - j) * 2))
                        } else {
                            (j, prev[i][j].0.saturating_add(j))
                        };
                        if x < next[i1][l1].0 {
                            next[i1][l1] = (x, Some((i, j, k)));
                        }

                        next
                    })
            });
        pushed!(dp, next)
    });

    let min_idx = dp[dp.len() - 1][0].citer().position_min().unwrap();

    let nums = dp
        .iter()
        .skip(1)
        .rev()
        .scan((0, min_idx), |(idx0, idx1), dp_row| {
            let (next_idx0, next_idx1, k) = dp_row[*idx0][*idx1].1.unwrap();
            // eprintln!("{:?} {:?}", dp_row[*idx0][*idx1], c[k]);
            *idx0 = next_idx0;
            *idx1 = next_idx1;
            Some(&c[k].1)
        })
        .fold((1, vec![]), |(d, nums), cc| {
            let len = max(nums.len(), cc.len());
            let nums = izip!(
                nums.into_iter().chain(repeat(0)),
                cc.citer().chain(repeat(0)),
            )
            .take(len)
            .map(|(x, y)| x + d * y)
            .collect_vec();
            (10 * d, nums)
        })
        .1;
    let ans = if nums.len() == 1 {
        nums[0].to_string()
    } else {
        nums.citer().join("+") + "="
    };
    println!("{}", ans);
}
