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
    let (n, m, k) = read_tuple!(usize, usize, usize);
    let d = read_str();
    let s = read_vec(n, || read_str());

    let start = (0..n)
        .find_map(|i| s[i].citer().position(|c| c == 'S').map(|j| (i, j)))
        .unwrap();
    let goal = (0..n)
        .find_map(|i| s[i].citer().position(|c| c == 'G').map(|j| (i, j)))
        .unwrap();

    let to_idx = |c: char| match c {
        'U' => 0,
        'D' => 1,
        'L' => 2,
        'R' => 3,
        _ => unreachable!(),
    };

    let mut nexts = (0..2 * k)
        .rev()
        .scan([1 << 60; 4], |ns, i| {
            ns[to_idx(d[i % k])] = i;
            Some(*ns)
        })
        .collect::<Vec<_>>();
    nexts.reverse();

    let mut q = BinaryHeap::new();
    let mut costs = vec![vec![1 << 50; m]; n];

    q.push(Reverse((0, start)));
    costs[start.0][start.1] = 0;

    while let Some(Reverse((cost, (i, j)))) = q.pop() {
        if (i, j) == goal {
            println!("{}", cost);
            return;
        }

        if cost != costs[i][j] {
            continue;
        }

        it![(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)]
            .enumerate()
            .map(|(idx, (di, dj))| (idx, (i.wrapping_add(di), j.wrapping_add(dj))))
            .filter(|&(_, (ni, nj))| ni < n && nj < m && s[ni][nj] != '#')
            .for_each(|(idx, (ni, nj))| {
                let nc = cost + nexts[cost % k][idx] - cost % k + 1;

                if nc < costs[ni][nj] {
                    q.push(Reverse((nc, (ni, nj))));
                    costs[ni][nj] = nc;
                }
            });
    }

    println!("-1");
}
