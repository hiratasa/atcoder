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

fn match_by_translation(s: &[Vec<char>], t: &[Vec<char>]) -> bool {
    let n = s.len();

    let i0 = s
        .iter()
        .position(|row| row.citer().any(|c| c == '#'))
        .unwrap();
    let j0 = s
        .iter()
        .map(|row| row.citer().position(|c| c == '#').unwrap_or(n))
        .min()
        .unwrap();

    let i1 = t
        .iter()
        .position(|row| row.citer().any(|c| c == '#'))
        .unwrap();
    let j1 = t
        .iter()
        .map(|row| row.citer().position(|c| c == '#').unwrap_or(n))
        .min()
        .unwrap();

    iproduct!(0..n, 0..n)
        .map(|(i, j)| {
            (
                s.get(i0 + i)
                    .unwrap_or(&vec!['.'; n])
                    .get(j0 + j)
                    .copied()
                    .unwrap_or('.'),
                t.get(i1 + i)
                    .unwrap_or(&vec!['.'; n])
                    .get(j1 + j)
                    .copied()
                    .unwrap_or('.'),
            )
        })
        .all(|(c, d)| c == d)
}

fn rotate(s: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = s.len();

    (0..n)
        .map(|i| (0..n).map(|j| s[n - j - 1][i]).collect())
        .collect()
}

fn main() {
    let n: usize = read();
    let s = read_vec(n, || read_str());
    let t = read_vec(n, || read_str());

    if (0..4)
        .scan(s, |s, _| {
            *s = rotate(&s);
            Some(s.clone())
        })
        .any(|s| match_by_translation(&s, &t))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
