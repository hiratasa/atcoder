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

fn print0(mat: &Vec<Vec<Option<u32>>>) {
    for row in mat {
        println!(
            "{}",
            row.citer()
                .map(|x| x.map_or('.', |x| (b'a' + x as u8) as char))
                .join("")
        );
    }
}

#[allow(dead_code)]
fn solve0(
    n: usize,
    m: usize,
    m_current: usize,
    i: usize,
    j: usize,
    c: u32,
    mat: &mut Vec<Vec<Option<u32>>>,
) -> bool {
    if j == n {
        if m_current != m {
            return false;
        }
        return solve0(n, m, 0, i + 1, 0, c, mat);
    }

    if i == n {
        if (0..n).all(|i| (0..n).filter_map(|j| mat[j][i]).dedup().count() == m) {
            print0(mat);
            return true;
        } else {
            return false;
        }
    }

    if m_current + (n - j) < m {
        return false;
    }

    if solve0(
        n,
        m,
        m_current + mat[i][j].is_some() as usize,
        i,
        j + 1,
        c,
        mat,
    ) {
        return true;
    }

    if mat[i][j].is_none() {
        mat[i][j] = Some(c);

        if j + 1 < n && mat[i][j + 1].is_none() {
            mat[i][j + 1] = Some(c);
            if solve0(n, m, m_current + 1, i, j + 2, c + 1, mat) {
                return true;
            }
            mat[i][j + 1] = None;
        }

        if i + 1 < n {
            assert!(mat[i + 1][j].is_none());
            mat[i + 1][j] = Some(c);
            if solve0(n, m, m_current + 1, i, j + 1, c + 1, mat) {
                return true;
            }
            mat[i + 1][j] = None;
        }

        mat[i][j] = None;
    }

    false
}

fn main() {
    let n: usize = read();

    if n == 2 {
        println!("-1");
        return;
    }

    if n == 3 {
        println!("aa.");
        println!("..b");
        println!("..b");
        return;
    }

    let r = vec![
        vec!["aabc", "ddbc", "efgg", "efhh"],
        vec!["..abc", "..abc", "ddeef", "ggh.f", "iihjj"],
        vec!["...abc", "...abc", "ddee.f", "..gghf", "iij.h.", "kkj.ll"],
        vec![
            "....abc", "....abc", "....def", "....def", "gghh..i", "jjkk..i", "llmmnn.",
        ],
    ];

    for i in 0..n / 4 - 1 {
        for j in 0..4 {
            print!("{}", repeat(".").take(i * 4).join(""));
            print!("{}", r[0][j]);
            println!("{}", repeat(".").take(n - (i + 1) * 4).join(""));
        }
    }
    for row in &r[n % 4] {
        print!("{}", repeat(".").take(4 * (n / 4 - 1)).join(""));
        println!("{}", row);
    }
}
