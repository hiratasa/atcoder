fn main() {
    let k = read::<usize>();

    let (zenhan, chukei, kouhan_rev) = successors(Some((0, k - 1)), |&(_, t)| {
        if t == 0 {
            None
        } else {
            (1..)
                .step_by(2)
                .take_while(|&i| i * i + 1 <= t)
                .last()
                .map(|i| (i, t - (i * i + 1)))
                .or_else(|| Some((0, t - 1)))
        }
    })
    .skip(1)
    .scan((0, 0), |(x0, y0), (i, _)| {
        let (zenhan, (x1, y1), kouhan) = if i == 0 {
            calc0(*x0, *y0)
        } else {
            calc((i as i64 + 1) / 2, *x0, *y0)
        };

        *x0 = x1;
        *y0 = y1;

        Some((zenhan, (x1, y1), kouhan))
    })
    .fold(
        (vec![(0, 0)], (0, 0), vec![]),
        |(mut t0, _, mut t1), (zenhan, chukei, kouhan)| {
            t0.extend(zenhan);
            t1.extend(kouhan.citer().rev());

            (t0, chukei, t1)
        },
    );

    let ans = zenhan
        .citer()
        .chain(once(chukei))
        .chain(kouhan_rev.citer().rev())
        .collect::<Vec<_>>();

    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}

fn calc0(x0: i64, y0: i64) -> (Vec<(i64, i64)>, (i64, i64), Vec<(i64, i64)>) {
    (vec![(x0 + 1, y0)], (x0 + 1, y0 + 1), vec![(x0, y0 + 1)])
}

fn calc(n: i64, x0: i64, y0: i64) -> (Vec<(i64, i64)>, (i64, i64), Vec<(i64, i64)>) {
    let (mut x, mut y) = (x0, y0);

    let mut zenhan = vec![];

    for _ in 0..n - 1 {
        zenhan.push((x + 1, y));
        zenhan.push((x + 1, y + 2 * n));
        zenhan.push((x + 2, y + 2 * n));
        zenhan.push((x + 2, y));
        x += 2;
    }

    zenhan.push((x + 1, y));
    zenhan.push((x + 1, y + 2 * n));

    x += 2;
    y += 2 * n;

    let (x1, y1) = (x, y);

    let mut kouhan = vec![];

    for _ in 0..n - 1 {
        kouhan.push((x, y - 1));
        kouhan.push((x - 2 * n, y - 1));
        kouhan.push((x - 2 * n, y - 2));
        kouhan.push((x, y - 2));
        y -= 2;
    }

    kouhan.push((x, y - 1));
    kouhan.push((x - 2 * n, y - 1));

    (zenhan, (x1, y1), kouhan)
}

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
