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
    let n: usize = read();
    let a = read_row::<i64>();
    let b = read_row::<i64>();
    let p = read_row::<usize>()
        .into_iter()
        .map(|pp| pp - 1)
        .collect_vec();
    let ok = (0..n).filter(|&i| i != p[i]).all(|i| a[i] > b[p[i]]);
    if !ok {
        println!("-1");
        return;
    }

    let cycles = (0..n)
        .fold((vec![], vec![false; n]), |(cycles, mut used), i| {
            if used[i] {
                (cycles, used)
            } else {
                let cycle = iterate(i, |&j| p[j])
                    .skip(1)
                    .take_while(|&j| j != i)
                    .fold(vec![i], |cycle, j| pushed!(cycle, j));
                for &j in &cycle {
                    used[j] = true;
                }

                (pushed!(cycles, cycle), used)
            }
        })
        .0;

    let ans = cycles
        .iter()
        .filter(|cycle| cycle.len() > 1)
        .map(|cycle| {
            let m = cycle.len();
            let mut idxs = (0..cycle.len()).collect::<BTreeSet<_>>();

            let mut q = (0..m)
                .map(|i| (a[cycle[i]] - a[cycle[(i + 1) % m]], i, (i + 1) % m))
                .collect::<BinaryHeap<_>>();

            let mut ops = vec![];

            while let Some((a_delta, idx0, idx1)) = q.pop() {
                if !idxs.contains(&idx0) || !idxs.contains(&idx1) {
                    continue;
                }

                assert!(a_delta >= 0);

                ops.push((cycle[idx0], cycle[idx1]));
                idxs.remove(&idx1);

                if idxs.len() >= 2 {
                    if let Some(&next_idx) =
                        idxs.range(idx0 + 1..).next().or_else(|| idxs.iter().next())
                    {
                        q.push((a[cycle[idx0]] - a[cycle[next_idx]], idx0, next_idx));
                    }
                }
            }

            ops
        })
        .flatten()
        .collect_vec();

    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x + 1, y + 1);
    }
}
