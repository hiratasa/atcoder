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

#[allow(dead_code)]
fn calc(s: &[Vec<bool>]) -> usize {
    let h = s.len();
    let w = s[0].len();

    let sums = once(vec![0; w + 1])
        .chain(s.iter().map(|row| {
            once(0)
                .chain(row.citer().map(|c| c as usize))
                .cumsum::<usize>()
                .collect::<Vec<_>>()
        }))
        .scan(vec![0; w + 1], |sum, row| {
            izip!(sum.iter_mut(), row.citer()).for_each(|(x, y)| *x += y);

            Some(sum.clone())
        })
        .collect::<Vec<_>>();

    iproduct!((0..=h).tuple_combinations(), (0..=w).tuple_combinations())
        .map(|((i0, i1), (j0, j1))| sums[i1][j1] + sums[i0][j0] - sums[i0][j1] - sums[i1][j0])
        .filter(|&x| x % 2 > 0)
        .count()
}

#[allow(dead_code)]
fn solve0(n: usize, m: usize) -> (Vec<Vec<Vec<bool>>>, usize) {
    let h = (1 << n) - 1;
    let w = (1 << m) - 1;

    if h == 1 && w == 1 {
        return (vec![vec![vec![true]]], 1);
    }

    if h > 1 {
        let (ss, _) = solve0(n - 1, m);
        let (ss2, _) = solve0(1, m);

        iproduct!(ss, ss2)
            .map(|(s, s2)| {
                assert!(s2.len() == 1);

                s.iter()
                    .chain(s2.iter())
                    .chain(s.iter())
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .map(|s| {
                let l = calc(&s);

                (s, l)
            })
            .fold((vec![], 0), |(ss, lmax), (s, l)| {
                if l > lmax {
                    (vec![s], l)
                } else if l == lmax {
                    (pushed!(ss, s), l)
                } else {
                    (ss, lmax)
                }
            })
    } else {
        assert!(n == 1);

        let (ss, _) = solve0(n, m - 1);

        iproduct!(ss, it![false, true])
            .map(|(s, c)| {
                assert!(s.len() == 1);

                vec![
                    s[0].citer()
                        .chain(once(c))
                        .chain(s[0].citer())
                        .collect::<Vec<_>>(),
                ]
            })
            .map(|s| {
                let l = calc(&s);

                (s, l)
            })
            .fold((vec![], 0), |(ss, lmax), (s, l)| {
                if l > lmax {
                    (vec![s], l)
                } else if l == lmax {
                    (pushed!(ss, s), l)
                } else {
                    (ss, lmax)
                }
            })
    }
}

#[allow(dead_code)]
fn solve1(n: usize, m: usize) -> (Vec<Vec<Vec<bool>>>, usize) {
    let h = (1 << n) - 1;
    let w = (1 << m) - 1;

    if h == 1 && w == 1 {
        return (vec![vec![vec![true]]], 1);
    }

    let (nmin, nmax) = (min(n, m), max(n, m));
    let lmax = (1 << nmin) * ((1 << nmin) - 1) * (1 << nmax) * (1 << nmax) / 8;

    if h > 1 {
        let (ss, _) = solve1(n - 1, m);
        let (ss2, _) = solve1(1, m);

        (
            iproduct!(ss, ss2)
                .map(|(s, s2)| {
                    assert!(s2.len() == 1);

                    s.iter()
                        .chain(s2.iter())
                        .chain(s.iter())
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .map(|s| {
                    let l = calc(&s);

                    (s, l)
                })
                .filter(|(_, l)| *l == lmax)
                .map(|(s, _)| s)
                // 1列のものは全部取る
                .take(if w == 1 { usize::MAX } else { 1 })
                .collect::<Vec<_>>(),
            lmax,
        )
    } else {
        assert!(n == 1);

        let (ss, _) = solve1(n, m - 1);

        (
            iproduct!(ss, it![false, true])
                .map(|(s, c)| {
                    assert!(s.len() == 1);

                    vec![
                        s[0].citer()
                            .chain(once(c))
                            .chain(s[0].citer())
                            .collect::<Vec<_>>(),
                    ]
                })
                .map(|s| {
                    let l = calc(&s);

                    (s, l)
                })
                .filter(|(_, l)| *l == lmax)
                .map(|(s, _)| s)
                .collect::<Vec<_>>(),
            lmax,
        )
    }
}

fn solve2(n: usize, m: usize) -> (Vec<Vec<bool>>, usize) {
    let h = (1usize << n) - 1;
    let w = (1usize << m) - 1;

    if h == 1 && w == 1 {
        return (vec![vec![true]], 1);
    }

    let (nmin, nmax) = (min(n, m), max(n, m));
    let lmax = (1 << nmin) * ((1 << nmin) - 1) * (1 << nmax) * (1 << nmax) / 8;

    let s = (0..h)
        .map(|i| {
            if i % 2 == 0 {
                vec![true; w]
            } else {
                fn t(z: usize, row: &mut [bool]) {
                    let w = row.len();
                    if w == 0 {
                        // NOP
                    } else if z == 0 {
                        row[w / 2] = true;
                    } else {
                        t(z - 1, &mut row[..w / 2]);
                        t(z - 1, &mut row[w / 2 + 1..]);
                    }
                }

                let mut row = vec![false; w];
                t(((i + 1) / 2).trailing_zeros() as usize, &mut row);

                row
            }
        })
        .collect::<Vec<_>>();

    // assert!(calc(&s) == lmax);

    (s, lmax)
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);

    let (s, l) = solve2(n, m);

    for row in s {
        println!("{}", row.citer().map(|x| x as usize).join(""));
    }
    eprintln!("{}", l);
}
