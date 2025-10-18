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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
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
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
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
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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

// calc sum[x=0 to n-1] floor((a+x*b)/c)
#[allow(dead_code)]
fn floor_sum(n: usize, mut a: usize, mut b: usize, c: usize) -> usize {
    let mut ret = 0;

    ret += a.div_euclid(c) * n;
    a = a.rem_euclid(c);

    ret += b.div_euclid(c) * n * (n - 1) / 2;
    b = b.rem_euclid(c);

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
    let t = read::<usize>();

    repeat_with(|| read_tuple!(usize, usize, usize, usize))
        .take(t)
        .map(|(a, b, c, d)| {
            // (a+b*i ～ a+c*i がdの倍数を含まないような0<iの個数)
            // = (a+b*i ～ a+c*i がdの倍数を含まないような0<i<ceil((d-1)/(c-b))の個数)
            // = ceil((d-1)/(c-b)) - 1 - (a+b*i ～ a+c*i がdの倍数を含むような0<i<ceil((d-1)/(c-b))の個数)
            // = ceil((d-1)/(c-b)) - 1 - (0<i<ceil((d-1)/(c-b)) に対して a+b*i ～ a+c*i に含まれるdの倍数の個数の和)
            // = ceil((d-1)/(c-b)) - 1 - (0<i<ceil((d-1)/(c-b)) に対して a+c*i 以下のdの倍数の個数の和)
            //                         + (0<i<ceil((d-1)/(c-b)) に対して a+b*i-1 以下のdの倍数の個数の和)

            let u = (d - 1 + c - b - 1) / (c - b);

            u - 1 - ((floor_sum(u, a, c, d) - a / d) - (floor_sum(u, a - 1, b, d) - (a - 1) / d))
        })
        .for_each(|ans| {
            println!("{}", ans);
        });
}
