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

#[derive(Clone, Copy, Debug)]
enum Op {
    Rotate,
    ReverseRotate,
    InvertX(i64),
    InvertY(i64),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            '1' => Ok(Op::Rotate),
            '2' => Ok(Op::ReverseRotate),
            '3' => {
                let p = s.split_whitespace().nth(1).unwrap().parse::<i64>().unwrap();
                Ok(Op::InvertX(p))
            }
            '4' => {
                let p = s.split_whitespace().nth(1).unwrap().parse::<i64>().unwrap();
                Ok(Op::InvertY(p))
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let n: usize = read();
    let xy = read_vec(n, || read_tuple!(i64, i64));

    let m: usize = read();
    let ops = read_vec(m, || read::<Op>());

    let q: usize = read();
    let ab = read_vec(q, || read_tuple!(usize, usize));

    let t = once(vec![vec![1i64, 0i64, 0i64], vec![0i64, 1i64, 0i64]])
        .chain(ops.citer().scan(
            vec![vec![1i64, 0i64, 0i64], vec![0i64, 1i64, 0i64]],
            |t, op| {
                let u = match op {
                    Op::Rotate => {
                        // ((0, 1, 0), (-1, 0, 0))
                        vec![t[1].clone(), t[0].citer().map(|a| -a).collect_vec()]
                    }
                    Op::ReverseRotate => {
                        // ((0, -1, 0), (1, 0, 0))
                        vec![t[1].citer().map(|a| -a).collect_vec(), t[0].clone()]
                    }
                    Op::InvertX(p) => {
                        // ((-1, 0, 2 * p), (0, 1, 0))
                        vec![vec![-t[0][0], -t[0][1], -t[0][2] + 2 * p], t[1].clone()]
                    }
                    Op::InvertY(p) => {
                        // ((1, 0, 0), (0, -1, 2 * p)
                        vec![t[0].clone(), vec![-t[1][0], -t[1][1], -t[1][2] + 2 * p]]
                    }
                };
                *t = u;

                Some(t.clone())
            },
        ))
        .collect_vec();

    ab.citer()
        .map(|(a, b)| {
            let b = b - 1;

            let tt = &t[a];

            (
                tt[0][0] * xy[b].0 + tt[0][1] * xy[b].1 + tt[0][2],
                tt[1][0] * xy[b].0 + tt[1][1] * xy[b].1 + tt[1][2],
            )
        })
        .for_each(|ans| println!("{} {}", ans.0, ans.1));
}
