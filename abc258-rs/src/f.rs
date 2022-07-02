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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
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
    let t = read::<usize>();
    let case = read_vec(t, || read_tuple!(usize, usize, usize, usize, usize, usize));

    case.citer()
        .map(|(b, k, sx, sy, gx, gy)| {
            let xs = it![
                sx,
                gx,
                sx / b * b,
                (sx + b - 1) / b * b,
                gx / b * b,
                (gx + b - 1) / b * b
            ]
            .sorted()
            .dedup()
            .collect::<Vec<_>>();
            let ys = it![
                sy,
                gy,
                sy / b * b,
                (sy + b - 1) / b * b,
                gy / b * b,
                (gy + b - 1) / b * b
            ]
            .sorted()
            .dedup()
            .collect::<Vec<_>>();

            let xidxs = xs
                .citer()
                .enumerate()
                .map(|(i, x)| (x, i))
                .collect::<FxHashMap<_, _>>();
            let yidxs = ys
                .citer()
                .enumerate()
                .map(|(i, x)| (x, i))
                .collect::<FxHashMap<_, _>>();

            let (sx, sy) = (xidxs[&sx], yidxs[&sy]);
            let (gx, gy) = (xidxs[&gx], yidxs[&gy]);

            let mut q = BinaryHeap::new();
            let mut costs = vec![vec![usize::MAX; ys.len()]; xs.len()];
            costs[sx][sy] = 0;
            q.push(Reverse((0, (sx, sy))));

            while let Some(Reverse((cost, (x, y)))) = q.pop() {
                if (x, y) == (gx, gy) {
                    return cost;
                }

                let xw = if ys[y] % b == 0 { 1 } else { k };

                let yw = if xs[x] % b == 0 { 1 } else { k };

                it![x.wrapping_sub(1), x + 1]
                    .filter(|&nx| nx < xs.len())
                    .for_each(|nx| {
                        let ncost = cost + xw * (xs[max(x, nx)] - xs[min(x, nx)]);

                        if ncost < costs[nx][y] {
                            costs[nx][y] = ncost;
                            q.push(Reverse((ncost, (nx, y))));
                        }
                    });

                it![y.wrapping_sub(1), y + 1]
                    .filter(|&ny| ny < ys.len())
                    .for_each(|ny| {
                        let ncost = cost + yw * (ys[max(y, ny)] - ys[min(y, ny)]);

                        if ncost < costs[x][ny] {
                            costs[x][ny] = ncost;
                            q.push(Reverse((ncost, (x, ny))));
                        }
                    });
            }

            unreachable!()
        })
        .for_each(|ans| {
            println!("{}", ans);
        })
}
