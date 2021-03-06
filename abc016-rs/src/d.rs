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
pub struct Vector(f64, f64);

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

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self * rhs.0, self * rhs.1)
    }
}

impl Vector {
    pub const EPS: f64 = 1e-10;

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn cross(self, rhs: Self) -> f64 {
        self.0 * rhs.1 - rhs.0 * self.1
    }

    pub fn norm(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn distance(self, rhs: Self) -> f64 {
        (self - rhs).norm()
    }

    pub fn is_zero(self) -> bool {
        self.norm() < Self::EPS
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        self.is_zero()
            || rhs.is_zero()
            || self.cross(rhs).abs() < Self::EPS * f64::max(self.norm(), rhs.norm())
    }

    pub fn normalize(self) -> Option<Self> {
        let n = self.norm();
        if n == 0.0 {
            None
        } else {
            Some(Vector(self.0 / n, self.1 / n))
        }
    }
}

type Point = Vector;

// line by 2 points
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Line(Point, Point);

#[allow(dead_code)]
impl Line {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self(Vector(x0, y0), Vector(x1, y1))
    }

    pub fn slope(self) -> Vector {
        self.1 - self.0
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        self.slope().is_parallel(rhs.slope())
    }

    pub fn contains(self, p: Vector) -> bool {
        (self.1 - p).is_parallel(self.0 - p)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct LineSegment(Vector, Vector);

#[allow(dead_code)]
impl LineSegment {
    pub const EPS: f64 = Vector::EPS;

    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self(Vector(x0, y0), Vector(x1, y1))
    }

    pub fn as_line(self) -> Line {
        Line(self.0, self.1)
    }

    pub fn slope(self) -> Vector {
        self.as_line().slope()
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        self.as_line().is_parallel(rhs.as_line())
    }

    pub fn contains(self, p: Point) -> bool {
        if self.as_line().contains(p) {
            let pp0 = self.0 - p;
            let pp1 = self.1 - p;
            pp0.dot(pp1) < Self::EPS * f64::max(pp0.norm(), pp1.norm())
        } else {
            false
        }
    }

    pub fn has_intersection(self, rhs: Self) -> bool {
        self.contains(rhs.0)
            || self.contains(rhs.1)
            || rhs.contains(self.0)
            || rhs.contains(self.1)
            || (self.slope().cross(rhs.0 - self.0).signum()
                * self.slope().cross(rhs.1 - self.0).signum()
                < 0.0
                && rhs.slope().cross(self.0 - rhs.0).signum()
                    * rhs.slope().cross(self.1 - rhs.0).signum()
                    < 0.0)
    }
}

fn main() {
    let (ax, ay, bx, by) = read_tuple!(f64, f64, f64, f64);

    let n: usize = read();

    let xy = read_vec(n, || read_tuple!(f64, f64));

    let l0 = LineSegment::new(ax, ay, bx, by);
    let ans = xy
        .citer()
        .cycle()
        .tuple_windows()
        .take(n)
        .map(|((x0, y0), (x1, y1))| LineSegment::new(x0, y0, x1, y1))
        // .inspect(|&l| eprintln!("{}", l0.has_intersection(l)))
        .filter(|&l| l0.has_intersection(l))
        .count()
        / 2
        + 1;
    println!("{}", ans);
}
