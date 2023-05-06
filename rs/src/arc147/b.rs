#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
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

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        mut p: [Usize1; n]
    }

    #[derive(Debug, Clone, Copy)]
    enum Op {
        A(usize),
        B(usize),
    }

    let mut ans = vec![];

    for i in 0..2 {
        let l = (n + 1 - i) / 2;
        for j in 0..l {
            for k in 0..l - j - 1 {
                if p[2 * k + i] % 2 == i && p[2 * (k + 1) + i] % 2 != i {
                    ans.push(Op::B(2 * k + i));
                    p.swap(2 * k + i, 2 * (k + 1) + i);
                }
            }
        }
    }
    for i in 0..n / 2 {
        if p[2 * i] % 2 != 0 && p[2 * i + 1] % 2 != 1 {
            ans.push(Op::A(2 * i));
            p.swap(2 * i, 2 * i + 1);
        }
    }
    for i in 0..2 {
        let l = (n + 1 - i) / 2;
        for j in 0..l {
            for k in 0..l - j - 1 {
                if p[2 * k + i] > p[2 * (k + 1) + i] {
                    ans.push(Op::B(2 * k + i));
                    p.swap(2 * k + i, 2 * (k + 1) + i);
                }
            }
        }
    }

    println!("{}", ans.len());
    for op in ans {
        match op {
            Op::A(idx) => println!("A {}", idx + 1),
            Op::B(idx) => println!("B {}", idx + 1),
        }
    }
}
