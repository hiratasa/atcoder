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

type Rational = num::rational::Ratio<i128>;

#[allow(dead_code)]
fn solve0(xyc: &[(Rational, Rational, usize)]) -> Option<usize> {
    let n = xyc.len();

    iproduct!(0..n, 0..n, 0..n, 0..n)
        .filter(|&(i, j, k, l)| i != j && i != k && i != l && j != k && j != l && k != l)
        .filter(|&(i, j, k, l)| {
            let (x1, y1, _) = xyc[i];
            let (x2, y2, _) = xyc[j];
            let (x3, y3, _) = xyc[k];
            let (x4, y4, _) = xyc[l];

            // i-j と k-l が平行
            (y2 - y1) * (x4 - x3) == (x2 - x1) * (y4 - y3)
        })
        .filter(|&(i, j, k, l)| {
            let (x1, y1, _) = xyc[i];
            let (x2, y2, _) = xyc[j];
            let (x3, y3, _) = xyc[k];
            let (x4, y4, _) = xyc[l];

            // i-j の中心点と k-l の中心点が異なり、それらを結ぶ線分がgradに直角
            let c1 = ((x1 + x2) / 2, (y1 + y2) / 2);
            let c2 = ((x3 + x4) / 2, (y3 + y4) / 2);

            let g = (x2 - x1, y2 - y1);
            let n = (c2.0 - c1.0, c2.1 - c1.1);

            c1 != c2 && g.0 * n.0 + g.1 * n.1 == Rational::from(0)
        })
        .map(|(i, j, k, l)| xyc[i].2 + xyc[j].2 + xyc[k].2 + xyc[l].2)
        .max()
}

fn main() {
    let n: usize = read();

    let xyc = read_vec(n, || read_tuple!(Rational, Rational, usize));

    // use rand::rngs::SmallRng;
    // use rand::SeedableRng;
    // let mut rng = SmallRng::seed_from_u64(42);

    // let xyc = read_vec(n, || {
    //     use rand::distributions::Distribution;
    //     let dist = rand::distributions::Uniform::new(0i128, 5);
    //     let dist_weight = rand::distributions::Uniform::new(0, 5);
    //     (
    //         Rational::from(dist.sample(&mut rng)),
    //         Rational::from(dist.sample(&mut rng)),
    //         dist_weight.sample(&mut rng),
    //     )
    // });

    // if xyc.citer().map(|(x, y, _)| (x, y)).sorted().dedup().count() != n {
    //     continue;
    // }

    let infos = iproduct!(0..n, 0..n)
        .filter(|&(i, j)| i < j)
        .map(|(i, j)| {
            let (x1, y1, c1) = xyc[i];
            let (x2, y2, c2) = xyc[j];

            (
                (
                    x2 == x1,
                    if x2 == x1 {
                        Rational::from(1)
                    } else {
                        (y2 - y1) / (x2 - x1)
                    },
                ),
                ((x2 + x1) / 2, (y2 + y1) / 2),
                c1 + c2,
            )
        })
        .fold(FxHashMap::default(), |mut map, (g, c, weight)| {
            map.entry(g).or_insert(vec![]).push((c, weight));
            map
        });

    let ans = infos
        .iter()
        .filter_map(|(g, v)| {
            v.citer()
                // center毎にunique化
                .sorted_by_key(|&(c, w)| (c, Reverse(w)))
                .group_by(|&(c, _w)| c)
                .into_iter()
                .map(|(_, mut it)| it.next().unwrap())
                // centerのgrad成分を抜き出し
                .map(|(c, weight)| {
                    if g.0 {
                        // y軸平行
                        (c.1, weight)
                    } else {
                        (c.0 + c.1 * g.1, weight)
                    }
                })
                .sorted_by_key(|&(c, w)| (c, Reverse(w)))
                .group_by(|&(c, _w)| c)
                .into_iter()
                .filter_map(|(_c, it)| {
                    let v = it.map(|t| t.1).collect::<Vec<_>>();

                    if v.len() < 2 {
                        None
                    } else {
                        Some(v[0] + v[1])
                    }
                })
                .max()
        })
        .max();
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }

    // let ans0 = solve0(&xyc);
    // if ans != ans0 {
    //     for (x, y, c) in xyc {
    //         eprintln!("{} {} {}", x, y, c);
    //     }
    //     eprintln!("{:?}", ans0);
    //     return;
    // }
}
