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

fn calc_adj_lis(x: usize, v: &[usize]) -> usize {
    once(x)
        .chain(v.citer())
        .tuple_windows()
        .map(|(x0, x1)| max(x0, x1) - min(x0, x1))
        .fold(vec![0], |mut t, x| {
            let idx = t
                .binary_search_by(|&y| y.cmp(&x).then(Ordering::Greater))
                .unwrap_err();
            if idx == t.len() {
                t.push(x);
            } else {
                t[idx] = x;
            }

            t
        })
        .len()
        - 1
}

#[allow(dead_code)]
fn solve0(n: usize, x: usize) -> (usize, Vec<usize>) {
    (1..=n)
        .filter(|&i| i != x)
        .permutations(n - 1)
        .map(|p| {
            let l = calc_adj_lis(x, &p);
            (l, p)
        })
        .max()
        .unwrap()
}

fn solve(n: usize, x: usize) -> (usize, Vec<usize>) {
    iproduct!(
        it![false, true],
        it![x, n / 2, n / 2 + 1, n / 2 + 2]
            .filter(|&y| y <= n)
            .sorted()
            .dedup()
    )
    .map(|(b0, p0)| {
        let init = it![x, p0].dedup().collect::<Vec<_>>();
        (0..)
            .try_fold(
                (init, vec![], p0, p0, 0, b0),
                |(mut p, mut q, l, r, d, b), _| {
                    let l = if x + 1 == l { x } else { l };
                    let r = if r + 1 == x { x } else { r };

                    if l == 1 {
                        if r + d + 1 <= n && r + d + 1 != x {
                            q.extend(r + 1..r + d + 1);
                            p.push(r + d + 1);

                            Ok((p, q, l, r + d + 1, d + 1, b))
                        } else if r + d + 2 <= n {
                            assert!(r + d + 2 != x);

                            q.extend(r + 1..r + d + 1);
                            p.push(r + d + 2);

                            Ok((p, q, l, r + d + 2, d + 2, b))
                        } else {
                            Err(p
                                .into_iter()
                                .chain(q.into_iter())
                                .chain((r + 1..=n).filter(|&y| y != x))
                                .collect::<Vec<_>>())
                        }
                    } else if r == n {
                        if l > d + 1 && l - d - 1 != x {
                            q.extend(l - d..l);
                            p.push(l - d - 1);

                            Ok((p, q, l - d - 1, r, d + 1, b))
                        } else if l > d + 2 {
                            assert!(l - d - 2 != x);
                            q.extend(l - d..l);
                            p.push(l - d - 2);

                            Ok((p, q, l - d - 2, r, d + 2, b))
                        } else {
                            Err(p
                                .into_iter()
                                .chain(q.into_iter())
                                .chain((1..l).filter(|&y| y != x))
                                .collect::<Vec<_>>())
                        }
                    } else if b {
                        p.push(r + 1);

                        let d = p[p.len() - 1] - p[p.len() - 2];
                        Ok((p, q, l, r + 1, d, !b))
                    } else {
                        p.push(l - 1);

                        let d = p[p.len() - 2] - p[p.len() - 1];
                        Ok((p, q, l - 1, r, d, !b))
                    }
                },
            )
            .unwrap_err()
    })
    .map(|p| {
        let l = calc_adj_lis(x, &p[1..]);
        (l, p)
    })
    .max()
    .unwrap()
}

fn main() {
    let (n, x) = read_tuple!(usize, usize);

    let ans = solve(n, x);
    // let ans0 = solve0(n, x);

    assert!((ans.1).len() == n, "{} {} {:?}", n, x, ans.1);

    println!("{}", (ans.1).citer().join(" "));
}
