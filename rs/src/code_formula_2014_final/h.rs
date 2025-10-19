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
use itertools::{Itertools, chain, iproduct, iterate, izip};
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
    let (n, s1, s2) = read_tuple!(usize, usize, usize);
    // let mut ab = read_vec(n, || read_tuple!(usize, usize));
    // let mut ab = (0..n).map(|i| (i, i)).collect::<Vec<_>>();

    use rand::Rng;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;
    let mut rng = SmallRng::seed_from_u64(42);
    let mut ab = repeat_with(|| (rng.random_range(1..50001), rng.random_range(1..50001)))
        .take(n)
        .collect::<Vec<_>>();
    let start = std::time::Instant::now();

    // 規模の違いが1以上s以下の2国間の数を求める
    let mut calc = |s: usize| {
        if s == 0 {
            return 0;
        }

        let t = (s as f64).sqrt().floor() as usize;

        const B: usize = 50000;

        ab.sort_unstable_by_key(|&(a, _b)| a);
        let buckets = ab.citer().fold(vec![vec![]; B + 1], |mut buckets, (a, b)| {
            buckets[b].push(a);
            buckets
        });
        let ret0 = (1..=t)
            .map(|i| {
                let u = s / i;

                (0..=B - i)
                    .map(|j| {
                        buckets[j + i]
                            .citer()
                            .scan((0, 0), |(l, r), b| {
                                while *r < buckets[j].len() && buckets[j][*r] < b {
                                    *r += 1;
                                }

                                while *l < *r && buckets[j][*l] + u < b {
                                    *l += 1;
                                }

                                Some(*r - *l)
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        ab.sort_unstable_by_key(|&(_a, b)| b);
        let buckets = ab.citer().fold(vec![vec![]; B + 1], |mut buckets, (a, b)| {
            buckets[a].push(b);
            buckets
        });
        let t2 = if t * t == s { t - 1 } else { t };
        let ret1 = (1..=t2)
            .map(|i| {
                let u = s / i;

                (0..=B - i)
                    .map(|j| {
                        buckets[j + i]
                            .citer()
                            .scan((0, 0), |(l, r), b| {
                                while *r < buckets[j].len() && buckets[j][*r] + t < b {
                                    *r += 1;
                                }

                                while *l < *r && buckets[j][*l] + u < b {
                                    *l += 1;
                                }

                                Some(*r - *l)
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        ret0 + ret1
    };

    let ans = calc(s2) - calc(s1 - 1);

    println!("{}", ans);

    eprintln!("{}ms", start.elapsed().as_millis());
}
