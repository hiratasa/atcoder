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

fn lower_cost(c: &Vec<usize>, w: usize) -> usize {
    c.citer()
        .enumerate()
        .map(|(idx, m)| {
            if m == 0 {
                0
            } else {
                let i = (idx / w) as i64;
                let j = (idx % w) as i64;

                let ni = ((m - 1) / w) as i64;
                let nj = ((m - 1) % w) as i64;

                ((i - ni).abs() + (j - nj).abs()) as usize
            }
        })
        .sum()
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);

    let c = read_mat::<usize>(h);

    let (i, j) = iproduct!(0..h, 0..w).find(|&(i, j)| c[i][j] == 0).unwrap();
    let c = c.iter().flatten().copied().collect_vec();

    let d = (1..=h * w - 1).chain(once(0)).collect_vec();

    let mut q = BinaryHeap::new();
    let mut costs = FxHashMap::default();

    let lc = lower_cost(&c, w);
    q.push(Reverse((lc, 0, i, j, c.clone())));
    costs.insert(c.clone(), lc);

    while let Some(Reverse((cost, raw_cost, ci, cj, mut v))) = q.pop() {
        if costs[&v] < cost {
            continue;
        }

        if v == d {
            println!("{}", cost);
            return;
        }

        it!((usize::MAX, 0), (0, usize::MAX), (1, 0), (0, 1))
            .map(|(di, dj)| (ci.wrapping_add(di), cj.wrapping_add(dj)))
            .filter(|&(ni, nj)| ni < h && nj < w)
            .for_each(|(ni, nj)| {
                let idx = ci * w + cj;
                let nidx = ni * w + nj;

                v.swap(idx, nidx);
                let ncost = raw_cost + 1 + lower_cost(&v, w);
                if
                /* ncost <= 24 && */
                ncost < costs.get(&v).copied().unwrap_or(usize::MAX) {
                    q.push(Reverse((ncost, raw_cost + 1, ni, nj, v.clone())));
                    costs.insert(v.clone(), ncost);
                }
                v.swap(idx, nidx);
            });
    }
}
