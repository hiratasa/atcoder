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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
    let (n, q) = read_tuple!(usize, usize);
    let a = read_row::<i64>();
    let b = read_row::<i64>();
    let pxy = read_vec(q, || read_tuple!(usize, i64, i64));

    let mut offsets = izip!(a.citer(), b.citer())
        .map(|(x, y)| x + y)
        .collect::<Vec<_>>();
    let mut t = izip!(a.citer(), b.citer())
        .map(|(x, y)| x - y)
        .collect::<Vec<_>>();

    let mut plus = (1..2 * n - 1)
        .filter(|&i| t[i] >= 0)
        .map(|i| (Reverse(t[i]), i))
        .collect::<BinaryHeap<_>>();
    let mut minus = (1..2 * n - 1)
        .filter(|&i| t[i] < 0)
        .map(|i| (t[i], i))
        .collect::<BinaryHeap<_>>();

    let mut numplus = plus.len();
    let mut numminus = minus.len();

    let mut sumoffset = offsets.citer().sum::<i64>();

    let mut sumplus = (1..2 * n - 1)
        .filter(|&i| t[i] >= 0)
        .map(|i| t[i])
        .sum::<i64>();
    let mut summinus = (1..2 * n - 1)
        .filter(|&i| t[i] < 0)
        .map(|i| t[i])
        .sum::<i64>();

    let fix = |t: &[i64],
               plus: &mut BinaryHeap<(Reverse<i64>, usize)>,
               minus: &mut BinaryHeap<(i64, usize)>| {
        while matches!(plus.peek(), Some(&(Reverse(x), i)) if t[i] != x) {
            plus.pop();
        }

        while matches!(minus.peek(), Some(&(x, i)) if t[i] != x) {
            minus.pop();
        }
    };

    let calc = |t: &[i64],
                plus: &BinaryHeap<(Reverse<i64>, usize)>,
                minus: &BinaryHeap<(i64, usize)>,
                numplus: usize,
                _numminus: usize,
                sumplus: i64,
                summinus: i64| {
        if numplus % 2 == 0 {
            t[0] + t[2 * n - 1] + sumplus - summinus
        } else if n == 1 {
            t[0] + t[2 * n - 1]
        } else {
            let (Reverse(x), _) = plus.peek().unwrap();
            let (y, _) = minus.peek().unwrap();

            t[0] + t[2 * n - 1] + (sumplus - x) - (summinus - y) + (x + y).abs()
        }
    };

    for (p, x, y) in pxy {
        let p = p - 1;

        sumoffset -= offsets[p];
        if p != 0 && p != 2 * n - 1 {
            if t[p] >= 0 {
                sumplus -= t[p];
                numplus -= 1;
            } else {
                summinus -= t[p];
                numminus -= 1;
            }
        }

        offsets[p] = x + y;
        t[p] = x - y;

        sumoffset += offsets[p];
        if p != 0 && p != 2 * n - 1 {
            if t[p] >= 0 {
                sumplus += t[p];
                numplus += 1;
                plus.push((Reverse(t[p]), p));
            } else {
                summinus += t[p];
                numminus += 1;
                minus.push((t[p], p));
            }
        }

        fix(&t, &mut plus, &mut minus);

        let ans = (sumoffset + calc(&t, &plus, &minus, numplus, numminus, sumplus, summinus)) / 2;

        println!("{}", ans);
    }
}
