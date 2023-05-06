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
    let xy = read_vec(n, || read_tuple!(usize, usize));

    if n == 1 {
        println!("0");
        return;
    }

    if n == 2 {
        println!(
            "{}",
            min(
                (max(xy[0].0, xy[1].0) - min(xy[0].0, xy[1].0))
                    * (max(xy[0].1, xy[1].1) - min(xy[0].1, xy[1].1)),
                (max(xy[0].0, xy[1].1) - min(xy[0].0, xy[1].1))
                    * (max(xy[0].1, xy[1].0) - min(xy[0].1, xy[1].0))
            )
        );

        return;
    }

    let mi = xy.citer().map(|(x, y)| min(x, y)).min().unwrap();
    let ma = xy.citer().map(|(x, y)| max(x, y)).max().unwrap();

    // miとmaを両方同じ色にする
    let ans0 = if xy.citer().any(|(x, y)| min(x, y) == mi && max(x, y) != ma)
        || xy.citer().filter(|&(x, y)| max(x, y) == ma).count() >= 2
    {
        let mi_idx = xy
            .citer()
            .enumerate()
            .position_min_by_key(|&(i, (x, y))| (min(x, y), max(x, y), i))
            .unwrap();
        let ma_idx = xy
            .citer()
            .enumerate()
            .position_max_by_key(|&(i, (x, y))| (max(x, y), min(x, y), i))
            .unwrap();

        let xy2 = xy
            .citer()
            .enumerate()
            .filter(|&(i, _)| i != mi_idx && i != ma_idx)
            .map(|(_i, (x, y))| (min(x, y), max(x, y)))
            .sorted()
            .collect::<Vec<_>>();
        let r0 = max(xy[mi_idx].0, xy[mi_idx].1);
        let r1 = min(xy[ma_idx].0, xy[ma_idx].1);
        (ma - mi)
            * xy2
                .citer()
                .chain(once((usize::MAX, usize::MAX)))
                .scan((min(r0, r1), max(r0, r1)), |(mi1, ma1), (x, y)| {
                    let mi2 = min(*mi1, x);
                    let ma2 = max(*ma1, xy2[n - 3].0);

                    *mi1 = min(*mi1, y);
                    *ma1 = max(*ma1, y);

                    Some(ma2 - mi2)
                })
                .min()
                .unwrap()
    } else {
        usize::MAX
    };

    // miとmaを異なる色にする
    let ans1 = {
        let mi_idx = xy
            .citer()
            .position_min_by_key(|&(x, y)| (min(x, y), Reverse(max(x, y))))
            .unwrap();
        let ma_idx = xy
            .citer()
            .position_max_by_key(|&(x, y)| (max(x, y), Reverse(min(x, y))))
            .unwrap();

        let xy2 = xy
            .citer()
            .enumerate()
            .filter(|&(i, _)| i != mi_idx && i != ma_idx)
            .map(|(_i, (x, y))| (min(x, y), max(x, y)))
            .sorted()
            .collect::<Vec<_>>();
        let r0 = max(xy[mi_idx].0, xy[mi_idx].1);
        let r1 = min(xy[ma_idx].0, xy[ma_idx].1);
        xy2.citer()
            .chain(once((usize::MAX, 0)))
            // r0: maと同色の最小値, r1: miと同色の最大値
            .scan((r0, r1), |(r0, r1), (x, y)| {
                // xをmaと同色にした場合の値
                // 残りは大きいほうをmaと同色, 小さいほうをmiと同色にする
                let t = (ma - min(x, *r0)) * (max(max(y, *r1), xy2.last().unwrap().0) - mi);

                // xとmiと同色にした場合で更新
                *r0 = min(*r0, y);
                *r1 = max(*r1, x);

                Some(t)
            })
            .min()
            .unwrap()
    };

    let ans = min(ans0, ans1);
    println!("{}", ans);
}
