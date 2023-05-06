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

fn check(s: &Vec<Vec<char>>, seq: &[usize]) -> bool {
    // let h = s.len();
    let w = s[0].len();

    let mut used = vec![false; w];
    let mut single = false;
    for i in 0..w {
        if used[i] {
            continue;
        }

        if let Some(j) = (i + 1..w).filter(|&j| !used[j]).find(|&j| {
            seq.citer()
                .map(|u| s[u][i])
                .eq(seq.citer().map(|u| s[u][j]).rev())
        }) {
            used[i] = true;
            used[j] = true;
        } else if !single
            && seq
                .citer()
                .map(|u| s[u][i])
                .eq(seq.citer().map(|u| s[u][i]).rev())
        {
            used[i] = true;
            single = true;
        } else {
            return false;
        }
    }

    true
}

fn solve(
    s: &Vec<Vec<char>>,
    seq: &mut [usize],
    next: usize,
    single: bool,
    used: &mut [bool],
) -> bool {
    let h = s.len();

    if 2 * next + single as usize == h {
        return check(s, seq);
    }

    let i = (0..h).find(|&i| !used[i]).unwrap();

    used[i] = true;
    for j in i + 1..h {
        if used[j] {
            continue;
        }

        used[j] = true;

        seq[next] = i;
        seq[h - 1 - next] = j;

        if solve(s, seq, next + 1, single, used) {
            return true;
        }

        used[j] = false;
    }

    if h % 2 > 0 && !single {
        seq[h / 2] = i;
        if solve(s, seq, next, true, used) {
            return true;
        }
    }

    used[i] = false;

    return false;
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);

    let s = read_vec(h, || read_str());
    assert!(s[0].len() == w);

    if solve(&s, &mut vec![0; h], 0, false, &mut vec![false; h]) {
        println!("YES");
    } else {
        println!("NO");
    }
}
