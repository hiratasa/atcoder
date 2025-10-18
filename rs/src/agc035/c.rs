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
    let n = read::<usize>();

    if n.is_power_of_two() {
        println!("No");
        return;
    }

    let mut edges = vec![];

    edges.push((1, 2));
    edges.push((2, 3));
    edges.push((3, 1 + n));
    edges.push((1 + n, 2 + n));
    edges.push((2 + n, 3 + n));

    let add01 = |edges: &mut Vec<(usize, usize)>, i: usize| {
        assert!(i % 4 == 0);

        edges.push((i, i + 1));
        edges.push((i + 1, 1 + n));
        edges.push((1 + n, i + n));
        edges.push((i + n, i + 1 + n));
    };

    let add2 = |edges: &mut Vec<(usize, usize)>, i: usize| {
        assert!(i % 4 == 0);

        edges.push((i + 2, 3));
        edges.push((i + n, i + 2 + n));
    };

    let add3 = |edges: &mut Vec<(usize, usize)>, i: usize| {
        assert!(i % 4 == 0);

        edges.push((i + 3, 2 + n));
        edges.push((i + n, i + 3 + n));
    };

    for i in (1..).map(|i| 4 * i).take_while(|&i| i + 3 < n) {
        add01(&mut edges, i);
        add2(&mut edges, i);
        add3(&mut edges, i);
    }

    if n == 3 {
        // NOP
    } else if n % 4 == 0 {
        let idx = n.trailing_zeros();

        let m0 = 1 << idx;
        let m1 = n ^ m0;

        edges.push((n, m0 + 1));
        edges.push((m1 + n, n + n));
    } else if n % 4 == 1 {
        add01(&mut edges, n / 4 * 4);
    } else if n % 4 == 2 {
        add01(&mut edges, n / 4 * 4);
        add2(&mut edges, n / 4 * 4);
    } else {
        add01(&mut edges, n / 4 * 4);
        add2(&mut edges, n / 4 * 4);
        add3(&mut edges, n / 4 * 4);
    }

    println!("Yes");
    for (x, y) in edges {
        println!("{} {}", x, y);
    }
}
