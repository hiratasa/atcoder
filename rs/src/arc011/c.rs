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
    let (first, last) = read_tuple!(String, String);

    let n: usize = read();

    let s = chain(
        read_vec(n, || read::<String>()).into_iter(),
        chain(once(last), once(first)),
    )
    .collect_vec();

    if s[n] == s[n + 1] {
        println!("0");
        println!("{}", s[n + 1]);
        println!("{}", s[n]);
        return;
    }

    let is_adj = |i: usize, j: usize| {
        izip!(s[i].chars(), s[j].chars())
            .filter(|(c1, c2)| c1 != c2)
            .count()
            == 1
    };

    let mut q = VecDeque::new();
    let mut parents = vec![None; n + 2];

    q.push_back(n + 1);
    while let Some(v) = q.pop_front() {
        if v == n {
            break;
        }

        (0..n + 1).filter(|&u| is_adj(v, u)).for_each(|u| {
            if parents[u].is_none() {
                parents[u] = Some(v);
                q.push_back(u);
            }
        });
    }

    if parents[n].is_none() {
        println!("-1");
    } else {
        let ans = successors(Some(n), |&prev| parents[prev]).collect_vec();
        println!("{}", ans.len() - 2);
        for a in ans.citer().rev() {
            println!("{}", s[a]);
        }
    }
}
