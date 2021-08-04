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

fn main() {
    let (n, m, q) = read_tuple!(usize, usize, usize);

    let abst = read_vec(m, || read_tuple!(usize, usize, usize, usize));

    let dep_times = abst.citer().enumerate().fold(
        vec![BTreeSet::new(); n],
        |mut dep_times, (i, (a, _b, s, _t))| {
            dep_times[a - 1].insert((s, i));
            dep_times
        },
    );

    let mut adjs = vec![vec![0; 2 * m]; 20];
    adjs[0] = abst
        .citer()
        .enumerate()
        .flat_map(|(i, (_a, b, _s, t))| {
            let next_v = dep_times[b - 1]
                .range((t, 0)..)
                .next()
                .map_or(2 * i + 1, |(_, j)| 2 * j);
            it![2 * i + 1, next_v]
        })
        .collect::<Vec<_>>();

    for i in 1..20 {
        for v in 0..2 * m {
            adjs[i][v] = adjs[i - 1][adjs[i - 1][v]];
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum Ans {
        City(usize),
        Bus(usize),
    }

    let time = |idx: usize| {
        if idx % 2 == 0 {
            abst[idx / 2].2
        } else {
            abst[idx / 2].3
        }
    };

    use Ans::*;

    (0..q)
        .map(|_| read_tuple!(usize, usize, usize))
        .map(|(x, y, z)| {
            let y = y - 1;

            let v0 = if let Some((_, idx)) = dep_times[y].range((x, 0)..).next() {
                2 * *idx
            } else {
                return City(y);
            };

            // eprintln!("{} {} {} {}", x, y, z, v0);

            if time(v0) >= z {
                return City(y);
            }

            let v = (0..20)
                .rev()
                .fold(v0, |v, i| if time(adjs[i][v]) < z { adjs[i][v] } else { v });

            assert!(time(v) < z);

            if v % 2 == 0 {
                Bus(v / 2)
            } else {
                City(abst[v / 2].1 - 1)
            }
        })
        .for_each(|ans| match ans {
            City(i) => {
                println!("{}", i + 1);
            }
            Bus(i) => {
                println!("{} {}", abst[i].0, abst[i].1);
            }
        });
}
