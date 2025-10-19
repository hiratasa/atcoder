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

fn main() {
    let (x, y, n) = read_tuple!(usize, usize, usize);
    let ab = read_vec(n, || read_tuple!(i64, i64));

    let z = (x + y) / n;
    let z = z as i64;

    let (plus, minus, offset) = ab.citer().fold(
        (vec![], vec![], 0),
        |(mut plus, mut minus, mut offset), (a, b)| {
            if a >= b {
                plus.push((z * b, a - b));
            } else {
                minus.push((z * a, b - a));
                offset -= z;
            }

            (plus, minus, offset)
        },
    );

    let tplus = plus
        .citer()
        .flat_map(|(c, d)| (1..=z).map(move |i| c + i * d))
        .sorted()
        .collect::<Vec<_>>();
    let tminus = minus
        .citer()
        .flat_map(|(c, d)| (1..=z).map(move |i| c + i * d))
        .sorted()
        .collect::<Vec<_>>();

    let mut q = chain(
        plus.citer().map(|(c, d)| (c, d, 1, 0)),
        minus.citer().map(|(c, d)| (c, d, -1, 0)),
    )
    .map(|(c, d, e, f)| Reverse((c, d, e, f)))
    .collect::<BinaryHeap<_>>();

    let mut ans = i64::MAX;
    let mut s = x as i64 + offset;
    let mut ma = chain(plus.citer(), minus.citer())
        .map(|(c, _)| c)
        .max()
        .unwrap();
    let mut sp = 0;
    let mut sm = 0;
    while let Some(Reverse((c, d, e, f))) = q.pop() {
        let ans1 = if s == 0 {
            ma - c
        } else if s > 0 {
            let idx = tplus
                .binary_search_by(|&t| t.cmp(&ma).then(Ordering::Less))
                .unwrap_err();
            if idx - sp >= s as usize {
                ma - c
            } else {
                tplus.get(s as usize + sp - 1).copied().unwrap_or(1 << 50) - c
            }
        } else {
            let idx = tminus
                .binary_search_by(|&t| t.cmp(&ma).then(Ordering::Less))
                .unwrap_err();
            if idx - sm >= (-s) as usize {
                ma - c
            } else {
                tminus
                    .get((-s) as usize + sm - 1)
                    .copied()
                    .unwrap_or(1 << 50)
                    - c
            }
        };
        ans = min(ans, ans1);

        if f == z {
            break;
        }

        q.push(Reverse((c + d, d, e, f + 1)));
        s -= e;
        ma = max(ma, c + d);
        if e > 0 {
            sp += 1;
        } else {
            sm += 1;
        }
    }

    println!("{}", ans);
}
