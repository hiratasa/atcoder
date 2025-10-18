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
    let (n, m, q) = read_tuple!(usize, usize, usize);
    let s = read_vec(n, || read_str());
    let query = read_vec(q, || read_tuple!(usize, usize, usize, usize));

    let num_verts = (0..n)
        .map(|i| {
            once(0)
                .chain((0..m).map(|j| (s[i][j] == '1') as usize))
                .cumsum::<usize>()
                .collect::<Vec<_>>()
        })
        .fold(vec![vec![0; m + 1]], |t, mut row| {
            izip!(t.last().unwrap().citer(), row.iter_mut()).for_each(|(x, y)| *y += x);

            pushed!(t, row)
        });
    let num_yoko = (0..n)
        .map(|i| {
            once(0)
                .chain(
                    (0..m)
                        .tuple_windows()
                        .map(|(j0, j1)| (s[i][j0] == '1' && s[i][j1] == '1') as usize),
                )
                .cumsum::<usize>()
                .collect::<Vec<_>>()
        })
        .fold(vec![vec![0; m]], |t, mut row| {
            izip!(t.last().unwrap().citer(), row.iter_mut()).for_each(|(x, y)| *y += x);

            pushed!(t, row)
        });
    let num_tate = (0..n)
        .tuple_windows()
        .map(|(i0, i1)| {
            once(0)
                .chain((0..m).map(|j| (s[i0][j] == '1' && s[i1][j] == '1') as usize))
                .cumsum::<usize>()
                .collect::<Vec<_>>()
        })
        .fold(vec![vec![0; m + 1]], |t, mut row| {
            izip!(t.last().unwrap().citer(), row.iter_mut()).for_each(|(x, y)| *y += x);

            pushed!(t, row)
        });

    query
        .citer()
        .map(|(i0, j0, i1, j1)| {
            let i0 = i0 - 1;
            let j0 = j0 - 1;

            let nv = num_verts[i1][j1] + num_verts[i0][j0] - num_verts[i0][j1] - num_verts[i1][j0];
            let nyoko =
                num_yoko[i1][j1 - 1] + num_yoko[i0][j0] - num_yoko[i0][j1 - 1] - num_yoko[i1][j0];
            let ntate =
                num_tate[i1 - 1][j1] + num_tate[i0][j0] - num_tate[i0][j1] - num_tate[i1 - 1][j0];

            nv - nyoko - ntate
        })
        .for_each(|ans| {
            println!("{}", ans);
        });
}
