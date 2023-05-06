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

fn main() {
    let n = read::<usize>();
    let xyd = read_vec(n, || read_tuple!(f64, f64, char));

    let map = xyd
        .citer()
        .fold(FxHashMap::default(), |mut map, (x, y, d)| {
            let (xmin, xmax, ymin, ymax) = map.entry(d).or_insert((
                std::f64::MAX,
                std::f64::MIN,
                std::f64::MAX,
                std::f64::MIN,
            ));
            *xmin = f64::min(*xmin, x);
            *xmax = f64::max(*xmax, x);
            *ymin = f64::min(*ymin, y);
            *ymax = f64::max(*ymax, y);

            map
        });

    let deltas = it![
        ('L', (-1.0, 0.0)),
        ('R', (1.0, 0.0)),
        ('U', (0.0, 1.0)),
        ('D', (0.0, -1.0))
    ]
    .collect::<FxHashMap<_, _>>();

    let mut candidates = vec![0.0];
    iproduct!(map.iter(), map.iter()).for_each(
        |((&d0, &(xmin0, xmax0, ymin0, ymax0)), (&d1, &(xmin1, xmax1, ymin1, ymax1)))| {
            let delta0 = deltas[&d0];
            let delta1 = deltas[&d1];

            if delta0.0 != delta1.0 {
                candidates.push(-(xmin0 - xmin1) / (delta0.0 - delta1.0));
                candidates.push(-(xmax0 - xmax1) / (delta0.0 - delta1.0));
            }
            if delta0.1 != delta1.1 {
                candidates.push(-(ymin0 - ymin1) / (delta0.1 - delta1.1));
                candidates.push(-(ymax0 - ymax1) / (delta0.1 - delta1.1));
            }
        },
    );

    let ans = candidates
        .citer()
        .filter(|&t| t >= 0.0)
        .map(|t| {
            let xmin = map
                .iter()
                .map(|(d, v)| v.0 + deltas[&d].0 * t)
                .min_by_key(|&z| ordered_float::OrderedFloat(z))
                .unwrap();
            let xmax = map
                .iter()
                .map(|(d, v)| v.1 + deltas[&d].0 * t)
                .max_by_key(|&z| ordered_float::OrderedFloat(z))
                .unwrap();
            let ymin = map
                .iter()
                .map(|(d, v)| v.2 + deltas[&d].1 * t)
                .min_by_key(|&z| ordered_float::OrderedFloat(z))
                .unwrap();
            let ymax = map
                .iter()
                .map(|(d, v)| v.3 + deltas[&d].1 * t)
                .max_by_key(|&z| ordered_float::OrderedFloat(z))
                .unwrap();

            (xmax - xmin) * (ymax - ymin)
        })
        .min_by_key(|&s| ordered_float::OrderedFloat(s))
        .unwrap();

    println!("{}", ans);
}
