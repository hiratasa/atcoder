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

trait IteratorExt: Iterator + Sized {
    fn fold_vec<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T);
    fn fold_vec2<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T);
    fn fold_vec3<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> T;
}
impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn fold_vec<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T),
    {
        self.fold(init, |mut v, item| {
            let (idx, t) = f(item);
            v[idx] = t;
            v
        })
    }
    fn fold_vec2<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T),
    {
        self.fold(init, |mut v, item| {
            let (idx, t) = f(&v, item);
            v[idx] = t;
            v
        })
    }
    fn fold_vec3<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> T,
    {
        self.fold(init, |mut v, item| {
            let t = f(&v, item);
            v.push(t);
            v
        })
    }
}

fn find_wins(b: &Vec<Vec<Option<bool>>>, t: bool) -> Vec<Vec<(usize, usize)>> {
    let r1 = b.iter().enumerate().flat_map(|(i, row)| {
        row.citer()
            .scan(0usize, |s, c| {
                if c == Some(t) {
                    *s += 1;
                    Some(*s)
                } else {
                    *s = 0;
                    Some(0)
                }
            })
            .enumerate()
            .filter(|&(_j, s)| s >= 5)
            .map(move |(j, _s)| (j - 4..=j).map(move |jj| (i, jj)).collect_vec())
    });

    let r2 = (0..19).flat_map(|j| {
        (0..19)
            .map(move |i| (i, b[i][j]))
            .scan(0usize, |s, (i, c)| {
                if c == Some(t) {
                    *s += 1;
                    Some((i, *s))
                } else {
                    *s = 0;
                    Some((i, 0))
                }
            })
            .filter(|&(_i, s)| s >= 5)
            .map(move |(i, _s)| (i - 4..=i).map(move |ii| (ii, j)).collect_vec())
    });

    let r3 = b
        .iter()
        .enumerate()
        .fold_vec3(vec![vec![0; 19]], |v, (i, row)| {
            chain(once(0usize), v[i].citer().take(18))
                .enumerate()
                .map(|(j, s)| if row[j] == Some(t) { s + 1 } else { 0 })
                .collect_vec()
        })
        .into_iter()
        .skip(1)
        .enumerate()
        .flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .filter(|&(_j, s)| s >= 5)
                .map(move |(j, _s)| (0..5).map(|k| (i - k, j - k)).collect_vec())
        });

    let r4 = b
        .iter()
        .enumerate()
        .fold_vec3(vec![vec![0; 19]], |v, (i, row)| {
            chain(v[i].citer().skip(1), once(0usize))
                .enumerate()
                .map(|(j, s)| if row[j] == Some(t) { s + 1 } else { 0 })
                .collect_vec()
        })
        .into_iter()
        .skip(1)
        .enumerate()
        .flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .filter(|&(_j, s)| s >= 5)
                .map(move |(j, _s)| (0..5).map(|k| (i - k, j + k)).collect_vec())
        });

    chain(chain(r1, r2), chain(r3, r4)).collect_vec()
}

fn validate_wins(wins: &Vec<Vec<(usize, usize)>>) -> bool {
    wins.iter()
        .cloned()
        .fold1(|intersect, ww| {
            intersect
                .into_iter()
                .filter(|idx| ww.contains(idx))
                .collect_vec()
        })
        .map(|intersect| !intersect.is_empty())
        .unwrap_or(true)
}

fn main() {
    let b = read_vec(19, || {
        read::<String>()
            .chars()
            .map(|c| match c {
                '.' => None,
                'o' => Some(true),
                'x' => Some(false),
                _ => unreachable!(),
            })
            .collect_vec()
    });

    let num_black = b
        .iter()
        .map(|row| row.citer().filter(|&c| c == Some(true)).count())
        .sum::<usize>();
    let num_white = b
        .iter()
        .map(|row| row.citer().filter(|&c| c == Some(false)).count())
        .sum::<usize>();

    if !(num_black == num_white || num_black == num_white + 1) {
        println!("NO");
        return;
    }

    let black_wins = find_wins(&b, true);
    let white_wins = find_wins(&b, false);

    if !validate_wins(&black_wins) || !validate_wins(&white_wins) {
        println!("NO");
        return;
    }

    if !black_wins.is_empty() && !white_wins.is_empty() {
        println!("NO");
        return;
    }

    if !black_wins.is_empty() && num_black == num_white {
        println!("NO");
        return;
    }

    if !white_wins.is_empty() && num_black == num_white + 1 {
        println!("NO");
        return;
    }

    println!("YES");
}
