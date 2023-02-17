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

fn main() {
    let (n, m, r) = read_tuple!(usize, usize, usize);

    let add = |x: usize, y: usize| {
        if r == 0 {
            if x + y >= n {
                0
            } else {
                x + y
            }
        } else {
            (x + y) % n
        }
    };

    let mut status = vec![vec![vec![vec![vec![vec![0; n]; n]; n]; n]; m + 1]; m + 1];
    let mut table = vec![vec![vec![vec![vec![vec![Err(0); n]; n]; n]; n]; m + 1]; m + 1];
    let mut stack = vec![];

    stack.push((true, m, m, 1, 1, 1, 1, None));

    while let Some((first, i0, i1, x0, y0, x1, y1, parent)) = stack.pop() {
        if first {
            if status[i0][i1][x0][y0][x1][y1] == 1 {
                // loop
                println!("Infinite");
                return;
            }

            if status[i0][i1][x0][y0][x1][y1] == 2 {
                // visited
                stack.push((false, i0, i1, x0, y0, x1, y1, parent));
                continue;
            }

            status[i0][i1][x0][y0][x1][y1] = 1;
            stack.push((false, i0, i1, x0, y0, x1, y1, parent));

            // select
            let v = [
                ((add(x1, x0), y1), x1),
                ((x1, add(y1, x0)), y1),
                ((add(x1, y0), y1), x1),
                ((x1, add(y1, y0)), y1),
            ];
            for (idx, (z, org)) in v.citer().enumerate() {
                if z != (x1, y1)
                    && org != 0
                    && v.citer().position(|(zz, _)| z == zz).unwrap() == idx
                {
                    stack.push((
                        true,
                        i1,
                        i0,
                        z.0,
                        z.1,
                        x0,
                        y0,
                        Some((i0, i1, x0, y0, x1, y1)),
                    ));
                }
            }

            // divide
            if i0 > 0 && min(x0, y0) == 0 && max(x0, y0) >= 2 {
                if x0 < y0 {
                    let k = y0;
                    for j in 1..k {
                        stack.push((
                            true,
                            i1,
                            i0 - 1,
                            x1,
                            y1,
                            j,
                            y0 - j,
                            Some((i0, i1, x0, y0, x1, y1)),
                        ));
                    }
                } else {
                    let k = x0;
                    for j in 1..k {
                        stack.push((
                            true,
                            i1,
                            i0 - 1,
                            x1,
                            y1,
                            x0 - j,
                            j,
                            Some((i0, i1, x0, y0, x1, y1)),
                        ));
                    }
                }
            }
        } else {
            status[i0][i1][x0][y0][x1][y1] = 2;

            if let Some(p) = parent {
                match table[i0][i1][x0][y0][x1][y1] {
                    Ok(k) => {
                        match table[p.0][p.1][p.2][p.3][p.4][p.5] {
                            Ok(_k1) => {
                                // NOP
                            }
                            Err(k1) => {
                                table[p.0][p.1][p.2][p.3][p.4][p.5] = Err(max(k + 1, k1));
                            }
                        }
                    }
                    Err(k) => match table[p.0][p.1][p.2][p.3][p.4][p.5] {
                        Ok(k1) => {
                            table[p.0][p.1][p.2][p.3][p.4][p.5] = Ok(min(k + 1, k1));
                        }
                        Err(_k1) => {
                            table[p.0][p.1][p.2][p.3][p.4][p.5] = Ok(k + 1);
                        }
                    },
                }
            }
        }
    }

    match table[m][m][1][1][1][1] {
        Ok(k) => {
            println!("First");
            println!("{}", k);
        }
        Err(k) => {
            println!("Second");
            println!("{}", k);
        }
    }
}
