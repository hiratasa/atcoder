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
    let (n, m) = read_tuple!(usize, usize);
    let abc = read_vec(n, || read_tuple!(i64, i64, i64));
    let def = read_vec(m, || read_tuple!(i64, i64, i64));

    let xs = chain(
        abc.citer().flat_map(|(a, b, _c)| it![a, b]),
        def.citer().map(|(d, _e, _f)| d),
    )
    .chain(it![-(1 << 50), 1 << 50, 0])
    .sorted()
    .dedup()
    .collect::<Vec<_>>();
    let x_idxs = xs
        .citer()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<FxHashMap<_, _>>();

    let ys = chain(
        abc.citer().map(|(_a, _b, c)| c),
        def.citer().flat_map(|(_d, e, f)| it![e, f]),
    )
    .chain(it![-(1 << 50), 1 << 50, 0])
    .sorted()
    .dedup()
    .collect::<Vec<_>>();
    let y_idxs = ys
        .citer()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<FxHashMap<_, _>>();

    let x_walls = abc
        .citer()
        .map(|(a, b, c)| (x_idxs[&a], x_idxs[&b], y_idxs[&c]))
        .fold(
            vec![vec![false; ys.len()]; xs.len()],
            |mut walls, (x0, x1, y)| {
                for x in x0..x1 {
                    walls[x][y] = true;
                }
                walls
            },
        );

    let y_walls = def
        .citer()
        .map(|(d, e, f)| (x_idxs[&d], y_idxs[&e], y_idxs[&f]))
        .fold(
            vec![vec![false; ys.len()]; xs.len()],
            |mut walls, (x, y0, y1)| {
                for y in y0..y1 {
                    walls[x][y] = true;
                }
                walls
            },
        );

    let mut stack = vec![];
    let mut visited = vec![vec![false; ys.len() - 1]; xs.len() - 1];

    stack.push((x_idxs[&0], y_idxs[&0]));
    visited[x_idxs[&0]][y_idxs[&0]] = true;

    while let Some((x, y)) = stack.pop() {
        if y > 0 && !x_walls[x][y] && !visited[x][y - 1] {
            stack.push((x, y - 1));
            visited[x][y - 1] = true;
        }

        if y < ys.len() - 2 && !x_walls[x][y + 1] && !visited[x][y + 1] {
            stack.push((x, y + 1));
            visited[x][y + 1] = true;
        }

        if x > 0 && !y_walls[x][y] && !visited[x - 1][y] {
            stack.push((x - 1, y));
            visited[x - 1][y] = true;
        }

        if x < xs.len() - 2 && !y_walls[x + 1][y] && !visited[x + 1][y] {
            stack.push((x + 1, y));
            visited[x + 1][y] = true;
        }
    }

    let ans = iproduct!((0..xs.len()).tuple_windows(), (0..ys.len()).tuple_windows())
        .filter(|&((x0, _x1), (y0, _y1))| visited[x0][y0])
        .try_fold(0, |s, ((x0, x1), (y0, y1))| {
            if x0 == 0 || x1 == xs.len() - 1 || y0 == 0 || y1 == ys.len() - 1 {
                None
            } else {
                Some(s + (xs[x1] - xs[x0]) * (ys[y1] - ys[y0]))
            }
        });

    if let Some(ans) = ans {
        println!("{}", ans)
    } else {
        println!("INF");
    }
}
