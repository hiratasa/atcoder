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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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

#[allow(dead_code)]
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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
    let (h, w, rs, cs) = read_tuple!(usize, usize, usize, usize);
    let n = read::<usize>();
    let rc = read_vec(n, || read_tuple!(usize, usize));
    let q = read::<usize>();
    let dl = read_vec(q, || read_tuple!(char, usize));

    let walls_r = rc.citer().fold(FxHashMap::default(), |mut walls, (r, c)| {
        walls.entry(r).or_insert(BTreeSet::new()).insert(c);
        walls
    });
    let walls_c = rc.citer().fold(FxHashMap::default(), |mut walls, (r, c)| {
        walls.entry(c).or_insert(BTreeSet::new()).insert(r);
        walls
    });

    dl.citer()
        .scan((rs, cs), |(r, c), (d, l)| {
            let (next_r, next_c) = match d {
                'L' => {
                    let t = walls_r
                        .get(r)
                        .and_then(|walls| walls.range(..*c).next_back())
                        .copied()
                        .unwrap_or(0);
                    (*r, max(t + 1, c.saturating_sub(l)))
                }
                'R' => {
                    let t = walls_r
                        .get(r)
                        .and_then(|walls| walls.range(*c + 1..).next())
                        .copied()
                        .unwrap_or(w + 1);
                    (*r, min(t - 1, *c + l))
                }
                'U' => {
                    let t = walls_c
                        .get(c)
                        .and_then(|walls| walls.range(..*r).next_back())
                        .copied()
                        .unwrap_or(0);
                    (max(t + 1, r.saturating_sub(l)), *c)
                }
                'D' => {
                    let t = walls_c
                        .get(c)
                        .and_then(|walls| walls.range(*r + 1..).next())
                        .copied()
                        .unwrap_or(h + 1);
                    (min(t - 1, *r + l), *c)
                }
                _ => unreachable!(),
            };

            *r = next_r;
            *c = next_c;

            Some((*r, *c))
        })
        .for_each(|ans| {
            println!("{} {}", ans.0, ans.1);
        });
}
