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

#[allow(dead_code)]
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
    let (h, w, h1, w1, h2, w2) = read_tuple!(usize, usize, usize, usize, usize, usize);
    let a = read_mat::<i64>(h);

    let (h2, w2) = (min(h1, h2), min(w1, w2));

    let s = once(vec![0; w + 1])
        .chain(a.iter().map(|row| {
            once(0)
                .chain(row.citer())
                .cumsum::<i64>()
                .collect::<Vec<_>>()
        }))
        .scan(vec![0; w + 1], |c, row| {
            izip!(c.iter_mut(), row.citer()).for_each(|(x, y)| *x += y);

            Some(c.clone())
        })
        .collect::<Vec<_>>();

    let s1 = (0..=h - h1)
        .map(|i| {
            (0..=w - w1)
                .map(|j| s[i + h1][j + w1] - s[i + h1][j] - s[i][j + w1] + s[i][j])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let s2 = (0..=h - h2)
        .map(|i| {
            (0..=w - w2)
                .map(|j| s[i + h2][j + w2] - s[i + h2][j] - s[i][j + w2] + s[i][j])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ww = w1 - w2 + 1;
    let t = (0..=h - h2)
        .map(|i| {
            (0..=w - w2)
                .scan(VecDeque::new(), |q, j| {
                    if j >= ww && matches!(q.front(), Some(&(_, k)) if k == j - ww) {
                        q.pop_front();
                    }

                    while matches!(q.back(), Some(&(v, _)) if v <= s2[i][j]) {
                        q.pop_back();
                    }

                    q.push_back((s2[i][j], j));

                    Some(q.front().copied().unwrap().0)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans = (0..=w - w1)
        .map(|j| {
            let j2 = j + ww - 1;

            let hh = h1 - h2 + 1;
            let q0 =
                (0..hh - 1)
                    .map(|i2| (t[i2][j2], i2))
                    .fold(VecDeque::new(), |mut q, (v, i2)| {
                        while matches!(q.back(), Some(&(u, _)) if u <= v) {
                            q.pop_back();
                        }

                        q.push_back((v, i2));

                        q
                    });
            (0..=h - h1)
                .scan(q0, |q, i| {
                    if i > 0 && matches!(q.front(), Some(&(_, k)) if k == i - 1) {
                        q.pop_front();
                    }

                    let v = t[i + hh - 1][j2];
                    while matches!(q.back(), Some(&(u, _)) if u <= v) {
                        q.pop_back();
                    }

                    q.push_back((v, i + hh - 1));

                    Some(s1[i][j] - q.front().unwrap().0)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
