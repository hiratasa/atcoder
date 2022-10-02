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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
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

fn main() {
    let n = read::<usize>();

    let ans = if n == 3 {
        vec![vec![5, 1, 9], vec![4, 8, 6], vec![2, 7, 3]]
    } else if n % 2 == 0 {
        chain(
            (0..n / 2).map(|i| {
                if i == 0 {
                    once(n * n)
                        .chain((2..).step_by(2).take(n - 1))
                        .collect::<Vec<_>>()
                } else {
                    (2 * n * i..).step_by(2).take(n).collect::<Vec<_>>()
                }
            }),
            (0..n / 2).map(|i| {
                (0..=2 * n - 1 + 2 * n * i)
                    .rev()
                    .step_by(2)
                    .take(n)
                    .collect::<Vec<_>>()
            }),
        )
        .collect::<Vec<_>>()
    } else {
        let mut evens = (1..=n * n).skip(1).step_by(2).collect::<FxHashSet<_>>();
        let mut odds = (1..=n * n).step_by(2).collect::<FxHashSet<_>>();

        let mut ans = vec![vec![0; n]; n];

        ans[n / 2][n / 2] = 1;
        assert!(odds.contains(&1));
        odds.remove(&1);

        ans[n / 2][n / 2 - 1] = n * n - 5;
        assert!(evens.contains(&(n * n - 5)));
        evens.remove(&(n * n - 5));

        ans[n / 2 + 1][n / 2 - 1] = 5;
        assert!(odds.contains(&5));
        odds.remove(&5);

        ans[n / 2 - 1][n / 2] = n * n - 1;
        assert!(evens.contains(&(n * n - 1)));
        evens.remove(&(n * n - 1));

        for i in 0..n / 2 - 1 {
            let x = if i == 0 { 2 * i + 3 } else { 2 * i + 5 };
            let y = n * n - x;

            assert!(evens.contains(&y));
            assert!(odds.contains(&x));
            ans[n / 2][i] = y;
            ans[n / 2 + 1][i] = x;

            evens.remove(&y);
            odds.remove(&x);
        }

        for i in n / 2 + 1..n {
            let x = 2 * i + 1;
            let y = n * n - x;

            assert!(evens.contains(&y));
            assert!(odds.contains(&x));
            ans[n / 2 - 1][i] = y;
            ans[n / 2][i] = x;

            evens.remove(&y);
            odds.remove(&x);
        }

        let mut evens = evens.citer().collect::<Vec<_>>();
        let mut odds = odds.citer().collect::<Vec<_>>();

        for i in 0..n {
            for j in 0..n {
                if ans[i][j] == 0 {
                    if i < n / 2 {
                        let x = evens.pop().unwrap();
                        ans[i][j] = x;
                    } else {
                        let x = odds.pop().unwrap();
                        ans[i][j] = x;
                    }
                }
            }
        }

        ans
    };

    use io::Write;
    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());
    for row in ans {
        writeln!(stdout, "{}", row.citer().join(" ")).unwrap();
    }
}
