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

use num::complex::Complex64;

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

// Calculate fft: g[i] = sum[j=0 to n] f[j] * c^(i * j)
// - length must be power of two
// - c^n == 1
// 計算量 O(n logn)
fn fft<
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num::One,
>(
    f: &mut Vec<T>,
    c: T,
) {
    let n = f.len();

    if n == 1 {
        return;
    }

    assert!(n.is_power_of_two());

    let d = n.trailing_zeros() as usize;

    for i in 0..n {
        let j = i.reverse_bits() >> (std::mem::size_of::<usize>() * 8 - d);

        if i < j {
            f.swap(i, j);
        }
    }

    let cs = std::iter::successors(Some(c), |cc| Some(*cc * *cc))
        .take(d)
        .collect::<Vec<_>>();
    for i in 0..d {
        let b = 1 << i;
        let c = n >> (i + 1); // b * c == n/2
        let z = cs[d - 1 - i];

        for j in 0..c {
            let mut p = T::one();

            for k in 0..b {
                let t1 = f[j * 2 * b + k];
                let t2 = f[j * 2 * b + k + b];
                f[j * 2 * b + k] = t1 + p * t2;
                f[j * 2 * b + k + b] = t1 - p * t2;
                p = p * z;
            }
        }
    }
}

#[allow(dead_code)]
fn fft_complex(f: &mut Vec<Complex64>) {
    let n = f.len();
    fft(
        f,
        Complex64::from_polar(1.0, 2.0 * std::f64::consts::PI / (n as f64)),
    );
}

#[allow(dead_code)]
fn inv_fft_complex(f: &mut Vec<Complex64>) {
    let n = f.len();

    fft(
        f,
        Complex64::from_polar(1.0, -2.0 * std::f64::consts::PI / (n as f64)),
    );
    for x in f {
        *x /= n as f64;
    }
}

#[allow(dead_code)]
fn convolution<T: Copy + num::ToPrimitive + num::FromPrimitive>(p: &Vec<T>, q: &Vec<T>) -> Vec<T> {
    let n0 = p.len();
    let n1 = q.len();

    let n = (n0 + n1 - 1).next_power_of_two();

    // T => f64 => Complex64
    let mut pf = p
        .iter()
        .map(|x| x.to_f64().unwrap().into())
        .chain(std::iter::repeat(Complex64::new(0.0, 0.0)))
        .take(n)
        .collect::<Vec<_>>();
    let mut qf = q
        .iter()
        .map(|x| x.to_f64().unwrap().into())
        .chain(std::iter::repeat(Complex64::new(0.0, 0.0)))
        .take(n)
        .collect::<Vec<_>>();

    fft_complex(&mut pf);
    fft_complex(&mut qf);

    for (x, y) in pf.iter_mut().zip(&qf) {
        *x *= *y;
    }

    inv_fft_complex(&mut pf);

    pf.iter()
        .map(|x| T::from_f64(x.re.round()).unwrap())
        .take(n0 + n1 - 1)
        .collect()
}

fn main() {
    let n: usize = read();

    let (a, b): (Vec<_>, Vec<_>) = (0..n).map(|i| read_tuple!(usize, usize)).unzip();

    let c = convolution(&a, &b);

    println!("0");
    for cc in c {
        println!("{}", cc);
    }
}
