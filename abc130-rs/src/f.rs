#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

use ordered_float::OrderedFloat;

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
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

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

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

fn main() {
    let n: usize = read();

    let xyd = read_vec(n, || read_tuple!(f64, f64, char));

    let xs = [(vec!['U', 'D'], 0.0), (vec!['R'], 1.0), (vec!['L'], -1.0)]
        .iter()
        .filter_map(|(dirs, delta)| {
            xyd.iter()
                .copied()
                .filter(|(_, _, d)| dirs.contains(d))
                .map(|(x, _, _)| x)
                .minmax()
                .into_option()
                .map(|(min, max)| (*delta, min, max))
        })
        .collect_vec();

    let ys = [(vec!['L', 'R'], 0.0), (vec!['U'], 1.0), (vec!['D'], -1.0)]
        .iter()
        .filter_map(|(dirs, delta)| {
            xyd.iter()
                .copied()
                .filter(|(_, _, d)| dirs.contains(d))
                .map(|(_, y, _)| y)
                .minmax()
                .into_option()
                .map(|(min, max)| (*delta, min, max))
        })
        .collect_vec();

    let calc_x = |t: f64| {
        let xx = xs
            .iter()
            .copied()
            .flat_map(|(delta, min, max)| chain(once(min + delta * t), once(max + delta * t)))
            .minmax()
            .into_option()
            .unwrap();
        xx.1 - xx.0
    };

    let calc_y = |t: f64| {
        let yy = ys
            .iter()
            .copied()
            .flat_map(|(delta, min, max)| chain(once(min + delta * t), once(max + delta * t)))
            .minmax()
            .into_option()
            .unwrap();
        yy.1 - yy.0
    };

    let ans = chain(
        once(0.0),
        chain(
            xs.iter().copied().tuple_combinations().flat_map(
                |((delta1, minx1, maxx1), (delta2, minx2, maxx2))| {
                    chain(
                        // minx1 + delta1 * t == minx2 + delta2 * t
                        once(-(minx1 - minx2) / (delta1 - delta2)),
                        // maxx1 + delta1 * t == maxx2 + delta2 * t
                        once(-(maxx1 - maxx2) / (delta1 - delta2)),
                    )
                },
            ),
            ys.iter()
                .tuple_combinations()
                .filter(|((delta1, _, _), (delta2, _, _))| delta1 != delta2)
                .flat_map(|((delta1, miny1, maxy1), (delta2, miny2, maxy2))| {
                    chain(
                        // miny1 + delta1 * t == miny2 + delta2 * t
                        once(-(miny1 - miny2) / (delta1 - delta2)),
                        // maxy1 + delta1 * t == maxy2 + delta2 * t
                        once(-(maxy1 - maxy2) / (delta1 - delta2)),
                    )
                }),
        ),
    )
    .filter(|&t| t >= 0.0)
    .map(|t| calc_x(t) * calc_y(t))
    .map_into::<OrderedFloat<f64>>()
    .min()
    .unwrap();

    println!("{}", ans);
}
