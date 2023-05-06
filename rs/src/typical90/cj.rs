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

fn solve(
    g: &[BitSet],
    a: &[usize],
    idx: usize,
    selected: &mut BitSet,
    forbidden: &BitSet,
    s: usize,
    ss: &mut [Option<BitSet>],
) -> Option<(BitSet, BitSet)> {
    let n = g.len();
    if idx == n {
        return None;
    }

    selected.set(idx, true);

    let s2 = s + a[idx];
    if let Some(t) = &ss[s2] {
        return Some((t.clone(), selected.clone()));
    }

    ss[s2] = Some(selected.clone());

    let forbidden2 = forbidden | &g[idx];
    for next_idx in (idx + 1..n).filter(|&i| !forbidden2[i]) {
        if let Some(x) = solve(g, a, next_idx, selected, &forbidden2, s + a[idx], ss) {
            return Some(x);
        }
    }

    selected.set(idx, false);

    None
}

fn main() {
    let (n, q) = read_tuple!(usize, usize);

    let a = read_row::<usize>();

    let xy = read_vec(q, || read_tuple!(usize, usize));

    let g = xy.citer().fold(vec![bitset!(n, 0); n], |mut g, (x, y)| {
        g[x - 1].set(y - 1, true);
        g
    });

    let (x, y) = (0..n)
        .try_fold(vec![None; 8889], |mut ss, first_idx| {
            if let Some(t) = solve(
                &g,
                &a,
                first_idx,
                &mut bitset!(n, 0),
                &bitset!(n, 0),
                0,
                &mut ss,
            ) {
                Err(t)
            } else {
                Ok(ss)
            }
        })
        .unwrap_err();

    println!("{}", x.count_ones());
    println!("{}", (0..n).filter(|&i| x[i]).map(|i| i + 1).join(" "));
    println!("{}", y.count_ones());
    println!("{}", (0..n).filter(|&i| y[i]).map(|i| i + 1).join(" "));
}
