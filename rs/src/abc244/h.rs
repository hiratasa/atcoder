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

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

fn main() {
    let q: usize = read();
    let query = read_vec(q, || read_tuple!(i64, i64, i64, i64));

    // convex hull trick
    let add = |x: i64, y: i64, lower: &mut BTreeSet<(i64, i64)>| {
        if let Some(&(x2, y2)) = lower.range(..(x, y)).next_back() {
            if let Some(&(x3, y3)) = lower.range((x, y)..).next() {
                let (dx2, dy2) = (x2 - x, y2 - y);
                let (dx3, dy3) = (x3 - x, y3 - y);

                if dx2 * dy3 - dx3 * dy2 >= 0 {
                    return;
                }
            }
        }

        lower.insert((x, y));

        while let Some(&(x2, y2)) = lower.range(..(x, y)).next_back() {
            if let Some(&(x3, y3)) = lower.range(..(x2, y2)).next_back() {
                let (dx1, dy1) = (x - x2, y - y2);
                let (dx3, dy3) = (x3 - x2, y3 - y2);

                if dx3 * dy1 - dx1 * dy3 < 0 {
                    break;
                }
                lower.remove(&(x2, y2));
            } else {
                break;
            }
        }

        while let Some(&(x2, y2)) = lower.range((x, y)..).skip(1).next() {
            if let Some(&(x3, y3)) = lower.range((x2, y2)..).skip(1).next() {
                let (dx1, dy1) = (x - x2, y - y2);
                let (dx3, dy3) = (x3 - x2, y3 - y2);

                if dx1 * dy3 - dx3 * dy1 < 0 {
                    break;
                }
                lower.remove(&(x2, y2));
            } else {
                break;
            }
        }
    };

    let mut lower = BTreeSet::new();
    let mut upper = BTreeSet::new();
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;
    let mut xy = vec![];
    for &(x, y, a, b) in &query {
        add(x, y, &mut lower);
        add(x, -y, &mut upper);
        min_x = min(min_x, x);
        max_x = max(max_x, x);
        min_y = min(min_y, y);
        max_y = max(max_y, y);
        xy.push((x, y));

        let (hull, bb) = if b > 0 {
            // upperはyが正負逆に入っている
            (&upper, -b)
        } else {
            (&lower, b)
        };

        let x0 = lower_bound_int(min_x, max_x, |xx| {
            let (x1, y1) = *hull.range((xx, std::i64::MIN)..).next().unwrap();
            if let Some(&(x2, y2)) = hull.range((x1, y1)..).skip(1).next() {
                let c1 = a * x1 + bb * y1;
                let c2 = a * x2 + bb * y2;

                if c1 < c2 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Greater
            }
        });
        // 下側凸包の一番右側でxが等しい点が複数ある場合があるが、この場合はそのうちyの最も小さい値のみが最大値の候補となるので問題なし
        // (左側はadd()の中で一番yが小さいもの以外は消される)
        let (ans_x, ans_y) = *hull.range((x0, std::i64::MIN)..).next().unwrap();
        let ans = a * ans_x + bb * ans_y;
        println!("{}", ans);
    }
}
