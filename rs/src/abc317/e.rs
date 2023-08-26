#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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
    let (h, w) = read_tuple!(usize, usize);
    let a = read_vec(h, || read_str());

    let mut blocked = (0..h)
        .map(|i| (0..w).map(|j| a[i][j] == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..h {
        let mut b = false;
        for j in 0..w {
            match a[i][j] {
                '>' => {
                    b = true;
                }
                '.' | 'S' | 'G' => {
                    // NOP
                }
                _ => {
                    b = false;
                }
            }
            if b {
                blocked[i][j] = true;
            }
        }

        let mut b = false;
        for j in (0..w).rev() {
            match a[i][j] {
                '<' => {
                    b = true;
                }
                '.' | 'S' | 'G' => {
                    // NOP
                }
                _ => {
                    b = false;
                }
            }
            if b {
                blocked[i][j] = true;
            }
        }
    }

    for j in 0..w {
        let mut b = false;
        for i in 0..h {
            match a[i][j] {
                'v' => {
                    b = true;
                }
                '.' | 'S' | 'G' => {
                    // NOP
                }
                _ => {
                    b = false;
                }
            }
            if b {
                blocked[i][j] = true;
            }
        }

        let mut b = false;
        for i in (0..h).rev() {
            match a[i][j] {
                '^' => {
                    b = true;
                }
                '.' | 'S' | 'G' => {
                    // NOP
                }
                _ => {
                    b = false;
                }
            }
            if b {
                blocked[i][j] = true;
            }
        }
    }

    let (si, sj) = a
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.citer().position(|c| c == 'S').map(|j| (i, j)))
        .unwrap();
    let (gi, gj) = a
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.citer().position(|c| c == 'G').map(|j| (i, j)))
        .unwrap();

    let mut q = VecDeque::new();
    let mut costs = vec![vec![usize::MAX; w]; h];

    q.push_back((si, sj));
    costs[si][sj] = 0;

    let chmin = |a: &mut usize, b: usize| {
        if *a <= b {
            false
        } else {
            *a = b;
            true
        }
    };

    while let Some((i, j)) = q.pop_front() {
        let c = costs[i][j];
        q.extend(
            it![(-1, 0), (1, 0), (0, -1), (0, 1)]
                .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
                .filter(|&(ni, nj)| ni < h && nj < w)
                .filter(|&(ni, nj)| !blocked[ni][nj])
                .filter(|&(ni, nj)| chmin(&mut costs[ni][nj], c + 1)),
        );
    }

    println_opt(Some(costs[gi][gj]).filter(|&c| c != usize::MAX));
}
