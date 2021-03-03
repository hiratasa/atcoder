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
    let (n, m, k) = read_tuple!(usize, usize, usize);

    let a = read_vec(n, || read_row::<usize>());

    let q: usize = read();
    let queries = read_vec(q, || read_tuple!(usize, usize, usize, usize, usize));

    let s = a
        .iter()
        .map(|row| {
            row.citer().fold(vec![vec![0]; k + 1], |mut b, t| {
                for bb in &mut b {
                    bb.push(*bb.last().unwrap());
                }

                *b[t].last_mut().unwrap() += 1;

                b
            })
        })
        .fold(vec![vec![vec![0; m + 1]]; k + 1], |mut b, row| {
            for i in 0..=k {
                let bb = b[i].last().unwrap().clone();
                b[i].push(bb);
                for j in 0..=m {
                    b[i].last_mut().unwrap()[j] += row[i][j];
                }
            }

            b
        });

    queries
        .citer()
        .scan((a, s), |(a, s), (t, x1, y1, x2, y2)| {
            if t == 1 {
                let x1 = x1 - 1;
                let x2 = x2 - 1;
                let y1 = y1 - 1;
                let y2 = y2 - 1;
                let a1 = a[x1][y1];
                let a2 = a[x2][y2];
                if x1 == x2 {
                    let x = x1;
                    let (amin, amax, ymin) = if y1 < y2 { (a1, a2, y1) } else { (a2, a1, y2) };

                    for xx in x..n {
                        s[amin][xx + 1][ymin + 1] -= 1;
                        s[amax][xx + 1][ymin + 1] += 1;
                    }
                } else {
                    let y = y1;
                    let (amin, amax, xmin) = if x1 < x2 { (a1, a2, x1) } else { (a2, a1, x2) };

                    for yy in y..m {
                        s[amin][xmin + 1][yy + 1] -= 1;
                        s[amax][xmin + 1][yy + 1] += 1;
                    }
                }

                a[x1][y1] = a2;
                a[x2][y2] = a1;

                Some(None)
            } else {
                // eprintln!("{:?}", s);
                let ans = (1..=k)
                    .map(|i| {
                        (
                            i,
                            s[i][x2][y2] + s[i][x1 - 1][y1 - 1]
                                - s[i][x2][y1 - 1]
                                - s[i][x1 - 1][y2],
                        )
                    })
                    .max_by_key(|&(t, m)| (m, t))
                    .unwrap();
                Some(Some(ans))
            }
        })
        .flatten()
        .for_each(|(x, y)| println!("{} {}", x, y));
}
