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
    let (n, m, k) = read_tuple!(usize, usize, usize);
    let a = read_row::<usize>();

    if k >= m && (0..=k - m).any(|i| a[i..i + m].citer().map(|aa| aa - a[i]).eq(0..m)) {
        println!("-1");
        return;
    }

    let (p, cp, ep, cep) = (1..=n + m).fold(
        (vec![1.0], vec![0.0, 1.0], vec![0.0], vec![0.0, 0.0]),
        |(mut p, mut cp, mut ep, mut cep), i| {
            let mut pp = (cp[min(i, n)] - cp[i.checked_sub(m).unwrap_or(0)]) / (m as f64);
            pp -= a
                .citer()
                .take_while(|&aa| aa < i)
                .skip_while(|&aa| aa < i.checked_sub(m).unwrap_or(0))
                .map(|aa| p[aa] / (m as f64))
                .sum::<f64>();

            p.push(pp);
            cp.push(cp[i] + pp);

            let mut ee = (cep[min(i, n)] - cep[i.checked_sub(m).unwrap_or(0)]) / (m as f64) + pp;
            ee -= a
                .citer()
                .take_while(|&aa| aa < i)
                .skip_while(|&aa| aa < i.checked_sub(m).unwrap_or(0))
                .map(|aa| ep[aa] / (m as f64))
                .sum::<f64>();

            ep.push(ee);
            cep.push(cep[i] + ee);

            (p, cp, ep, cep)
        },
    );
    // eprintln!("{:?} {:?} {:?} {:?}", p, cp, ep, cep);

    let pp = cp[n + m] - cp[n];
    let ee = cep[n + m] - cep[n];
    let e = ee / pp;

    let qf = a.citer().map(|aa| ep[aa]).sum::<f64>();

    // sum[i=0 to infinity] pq^i (e+i*f)
    // = e + q/p * f
    // (q = 1 - p)
    let ans = e + qf / pp;
    println!("{}", ans);
}
