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

fn calc<'a>(
    v: &'a Vec<usize>,
    w: &'a String,
    r: usize,
    next: usize,
    assign: &mut Vec<Option<&'a str>>,
    candidates: &mut FxHashSet<Vec<Option<&'a str>>>,
) {
    if w.len() - next < v.len() - r {
        return;
    }

    if w.len() - next > 3 * (v.len() - r) {
        return;
    }

    if w.len() == next {
        assert!(v.len() == r);
        candidates.insert(assign.clone());
        return;
    }

    for m in (next + 1..=next + 3).take_while(|&m| m <= w.len()) {
        if let Some(a) = assign[v[r]] {
            if a != w.get(next..m).unwrap() {
                continue;
            }
        }

        let tmp = assign[v[r]];
        assign[v[r]] = Some(w.get(next..m).unwrap());
        calc(v, w, r + 1, m, assign, candidates);
        assign[v[r]] = tmp;
    }
}

fn check_intersection<'a>(
    c0: &FxHashSet<Vec<Option<&'a str>>>,
    c1: &FxHashSet<Vec<Option<&'a str>>>,
) -> FxHashSet<Vec<Option<&'a str>>> {
    assert!(!c0.is_empty());
    assert!(!c1.is_empty());

    let cc0 = c0.iter().next().unwrap();
    let cc1 = c1.iter().next().unwrap();

    let both_used = izip!(cc0, cc1)
        .map(|(o0, o1)| o0.is_some() && o1.is_some())
        .collect_vec();

    let d1 = c1
        .iter()
        .map(|cc1| {
            cc1.iter()
                .copied()
                .enumerate()
                .filter(|&(i, _)| both_used[i])
                .map(|t| t.1)
                .collect_vec()
        })
        .collect::<FxHashSet<_>>();

    c0.iter()
        .filter(|cc0| {
            let dd0 = cc0
                .iter()
                .copied()
                .enumerate()
                .filter(|&(i, _)| both_used[i])
                .map(|t| t.1)
                .collect_vec();
            d1.contains(&dd0)
        })
        .cloned()
        .collect()
}

fn main() {
    let (k, n) = read_tuple!(usize, usize);

    let vw = read_vec(n, || read_tuple!(String, String));
    let vw = vw
        .iter()
        .map(|(v, w)| {
            (
                v.chars()
                    .map(|d| d.to_digit(10).unwrap() as usize)
                    .collect_vec(),
                w,
            )
        })
        .collect_vec();

    let c = vw
        .iter()
        .map(|(v, w)| {
            let mut candidates = FxHashSet::default();
            calc(&v, &w, 0, 0, &mut vec![None; k + 1], &mut candidates);
            candidates
        })
        .collect_vec();

    let d = c
        .iter()
        .map(|cc| {
            c.iter()
                .fold(cc.clone(), |cc, cc1| check_intersection(&cc, &cc1))
        })
        .collect_vec();
    let ans = d
        .iter()
        .multi_cartesian_product()
        .find_map(|assigns| {
            assigns.iter().copied().try_fold(
                vec![None; k + 1],
                |assign: Vec<Option<&str>>, next_assign: &Vec<Option<&str>>| {
                    izip!(&assign, next_assign)
                        .map(|(a0, a1)| {
                            if let Some(b0) = a0 {
                                if let Some(b1) = a1 {
                                    if b0 != b1 {
                                        return None;
                                    }
                                }
                            }
                            Some(a0.or(*a1))
                        })
                        .collect::<Option<Vec<_>>>()
                },
            )
        })
        .unwrap();
    for ans in ans.iter().skip(1) {
        println!("{}", ans.unwrap());
    }
}
