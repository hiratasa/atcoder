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

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Vector(i64, i64);

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<Vector> for i64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self * rhs.0, self * rhs.1)
    }
}

impl Vector {
    pub fn cross(self, rhs: Self) -> i64 {
        self.0 * rhs.1 - rhs.0 * self.1
    }
}

#[allow(dead_code)]
fn convex_hull(points: &Vec<Vector>) -> (Vec<usize>, Vec<usize>) {
    assert!(points.len() > 2);

    let idxs = {
        let mut idxs = (0..points.len()).collect::<Vec<_>>();
        idxs.sort_by(|&idx1, &idx2| points[idx1].partial_cmp(&points[idx2]).unwrap());
        idxs
    };

    let lower_ch =
        idxs.iter()
            .map(|&i| (i, &points[i]))
            .fold(vec![], |mut ch: Vec<usize>, (i, p)| {
                while ch.len() >= 2
                    && (points[ch[ch.len() - 1]] - points[ch[ch.len() - 2]])
                        .cross(*p - points[ch[ch.len() - 2]])
                        <= 0
                {
                    ch.pop();
                }

                ch.push(i);

                ch
            });
    let upper_ch = idxs.iter().rev().map(|&i| (i, &points[i])).skip(1).fold(
        vec![*lower_ch.last().unwrap()],
        |mut ch: Vec<usize>, (i, p)| {
            while ch.len() >= 2
                && (points[ch[ch.len() - 1]] - points[ch[ch.len() - 2]])
                    .cross(*p - points[ch[ch.len() - 2]])
                    <= 0
            {
                ch.pop();
            }

            ch.push(i);

            ch
        },
    );

    (lower_ch, upper_ch)
}

/// calc sum[x=0 to n-1] floor((a+x*b)/c)
#[allow(dead_code)]
fn floor_sum(n: i64, mut a: i64, mut b: i64, c: i64) -> i64 {
    let mut ret = 0;

    ret += a.div_euclid(c) * n;
    a = a.rem_euclid(c);
    assert!(a >= 0);

    ret += b.div_euclid(c) * n * (n - 1) / 2;
    b = b.rem_euclid(c);
    assert!(b >= 0);

    if b == 0 {
        return ret;
    }

    // y=(a+x*b)/c に対して、以下のように変数変換して考える
    //  x' = floor((a+n*b)/c) - y
    //  y' = n - x
    //  => y ' = ((a+n*b)%c+x'*c)/b
    // これに対するfloor_sumは元の式に対するfloor_sumと一致する(省略; グラフで格子点の数を考える)
    let last = a + n * b;
    ret += floor_sum(last / c, last % c, c, b);
    ret
}

fn main() {
    let n: usize = read();
    let xy = read_vec(n, || read_tuple!(i64, i64));
    let points = xy.citer().map(|(x, y)| Vector(x, y)).collect::<Vec<_>>();

    let (lower_ch, upper_ch) = convex_hull(&points);

    let a0 = lower_ch
        .citer()
        .map(|i| xy[i])
        .tuple_windows()
        .map(|((x0, y0), (x1, y1))| {
            if x0 == x1 {
                return -(y1 - y0);
            }

            // sum[t=0 to x1-x0-1] ceil(y0 + (y1 - y0)/(x1 - x0) * t)
            // = - sum[t=0 to x1-x0-1] floor(- y0 - (y1 - y0)/(x1 - x0) * t)
            -floor_sum(x1 - x0, -y0 * (x1 - x0), -(y1 - y0), x1 - x0)
        })
        .sum::<i64>();

    let a1 = upper_ch
        .citer()
        .rev()
        .map(|i| xy[i])
        .tuple_windows()
        .map(|((x0, y0), (x1, y1))| {
            if x0 == x1 {
                return 0;
            }

            // sum[t=0 to x1-x0-1] [floor(y0 + (y1 - y0)/(x1 - x0) * t) + 1]
            floor_sum(x1 - x0, y0 * (x1 - x0), y1 - y0, x1 - x0) + (x1 - x0)
        })
        .sum::<i64>();

    eprintln!("{} {}", a0, a1);

    // 一番右側の点は入ってない
    let ans = a1 - a0 - (n - 1) as i64;
    println!("{}", ans);
}
