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

use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

fn check(edges: &[Vec<usize>]) -> usize {
    let n: usize = edges.len();

    let mut dists = vec![vec![usize::MAX; n]; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        dists[i][i] = 0;
        q.clear();

        q.push_back((i, 0));

        while let Some((j, c)) = q.pop_front() {
            if c > dists[i][j] {
                continue;
            }

            for k in edges[j].citer() {
                if c + 1 < dists[i][k] {
                    dists[i][k] = c + 1;
                    q.push_back((k, c + 1));
                }
            }
        }
    }

    iproduct!(0..n, 0..n)
        .filter(|&(i, j)| dists[i][j] > 4)
        .count()
}

fn main() {
    let mut rng = SmallRng::from_entropy();

    let (n0, m0) = read_tuple!(usize, usize);
    let ab = read_vec(m0, || read_tuple!(usize, usize));

    let invert = |g: &mut [Vec<bool>], edges: &mut [Vec<usize>], i: usize, j: usize| {
        if g[i][j] {
            g[i][j] = false;
            g[j][i] = false;
            let idx = edges[i].citer().position(|k| k == j).unwrap();
            edges[i].remove(idx);
            let idx = edges[j].citer().position(|k| k == i).unwrap();
            edges[j].remove(idx);
        } else {
            g[i][j] = true;
            g[j][i] = true;
            edges[i].push(j);
            edges[j].push(i);
        }
    };

    let mut adjs = vec![vec![false; n0]; n0];
    let mut edges = vec![vec![]; n0];
    for (a, b) in ab {
        invert(&mut adjs, &mut edges, a - 1, b - 1);
    }

    let mut penalty = (0..n0).filter(|&i| edges[i].len() > 4).count() * (1 << 30) + check(&edges);
    for n in n0.. {
        let mut buf = vec![];
        let dist = Uniform::new(0, n);
        let start = std::time::Instant::now();

        for _i_itr in 0.. {
            if _i_itr > 0 && _i_itr % 100000 == 0 {
                eprintln!(
                    "#n={}, itr={}, elapsed={}ms",
                    n,
                    _i_itr,
                    start.elapsed().as_millis()
                );
            }

            let m = min(rng.gen_range(1, 10), n * (n - 1));

            buf.clear();
            buf.extend(
                repeat_with(|| (dist.sample(&mut rng), dist.sample(&mut rng)))
                    .filter(|&(i, j)| i != j)
                    .filter(|&(i, j)| adjs[i][j] || (edges[i].len() < 4 && edges[j].len() < 4))
                    .take(m),
            );

            for &(i, j) in &buf {
                invert(&mut adjs, &mut edges, i, j);
            }

            let new_penalty =
                (0..n).filter(|&i| edges[i].len() > 4).count() * (1 << 30) + check(&edges);
            let adopt = new_penalty <= penalty;
            if !adopt {
                for &(i, j) in buf.iter().rev() {
                    invert(&mut adjs, &mut edges, i, j);
                }
                continue;
            }
            penalty = new_penalty;

            if penalty == 0 {
                break;
            }
        }

        {
            let adjs = &adjs;
            let edges = (0..n)
                .flat_map(|i| (i + 1..n).filter(move |&j| adjs[i][j]).map(move |j| (i, j)))
                .collect::<Vec<_>>();

            println!("{} {}", n, edges.len());
            for (i, j) in edges {
                println!("{} {}", i + 1, j + 1);
            }
        }

        eprintln!("================");
        adjs.iter_mut().for_each(|row| row.resize(n + 1, false));
        adjs.resize(n + 1, vec![false; n + 1]);
        edges.resize(n + 1, vec![]);
        penalty += n;
    }
}
