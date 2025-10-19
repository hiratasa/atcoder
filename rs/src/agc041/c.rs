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

fn dfs(n: usize, grid: &mut [Vec<char>], i: usize, j: usize, limit: Option<usize>) -> bool {
    if i == n {
        assert!(limit.is_some());
        return (0..n).all(|jj| {
            (0..n)
                .map(|ii| grid[ii][jj])
                .dedup()
                .filter(|c| c.is_ascii_alphabetic())
                .count()
                == limit.unwrap()
        });
    }

    if j == n {
        let m = grid[i]
            .citer()
            .dedup()
            .filter(|c| c.is_ascii_alphabetic())
            .count();

        if m == 0 {
            return false;
        }

        if matches!(limit, Some(limit) if limit != m) {
            return false;
        }

        return dfs(n, grid, i + 1, 0, Some(m));
    }

    // 列のカウントをチェック(既に(i,j)に置かれている分も含む)
    let m = (0..=i)
        .map(|ii| grid[ii][j])
        .dedup()
        .filter(|c| c.is_ascii_alphabetic())
        .count();

    if grid[i][j] != '.' {
        if dfs(n, grid, i, j + 1, limit) {
            return true;
        }
    } else {
        if dfs(n, grid, i, j + 1, limit) {
            return true;
        }

        if limit.is_none() || m < limit.unwrap() {
            // 使う文字
            let t = iproduct!(-1i64..=2i64, -1i64..=2i64)
                .map(|(di, dj)| ((i as i64 + di, j as i64 + dj)))
                .filter(|&(ni, nj)| 0 <= ni && ni < n as i64 && 0 <= nj && nj < n as i64)
                .map(|(ni, nj)| (ni as usize, nj as usize))
                .map(|(ni, nj)| grid[ni][nj])
                .filter(|&c| c.is_ascii_alphabetic())
                .sorted()
                .dedup()
                .chain(once('#'))
                .enumerate()
                .find(|&(idx, c)| idx + 'a' as usize != c as usize)
                .map(|(idx, _)| (idx + 'a' as usize) as u8 as char)
                .unwrap();

            // 縦
            if i < n - 1 {
                grid[i][j] = t;
                grid[i + 1][j] = t;
                if dfs(n, grid, i, j + 1, limit) {
                    return true;
                }
                grid[i + 1][j] = '.';
                grid[i][j] = '.';
            }

            // 横
            if j < n - 1 && grid[i][j + 1] == '.' {
                grid[i][j] = t;
                grid[i][j + 1] = t;
                if dfs(n, grid, i, j + 1, limit) {
                    return true;
                }
                grid[i][j + 1] = '.';
                grid[i][j] = '.';
            }
        }
    }

    false
}

fn main() {
    let n = read::<usize>();

    if n <= 2 {
        println!("-1");
        return;
    }

    let ans = if n == 3 {
        let mut ans = vec![vec!['.'; n]; n];
        let ok = dfs(n, &mut ans, 0, 0, None);
        assert!(ok);
        ans
    } else {
        let mut ans4 = vec![vec!['.'; 4]; 4];
        let ok = dfs(4, &mut ans4, 0, 0, Some(3));
        assert!(ok);
        let ans4 = &ans4;

        if n % 4 == 0 {
            (0..n / 4)
                .flat_map(|i| {
                    (0..4).map(move |j| {
                        itertools::repeat_n('.', i * 4)
                            .chain(ans4[j].citer())
                            .chain(repeat_n('.', n - i * 4 - 4))
                            .collect::<Vec<_>>()
                    })
                })
                .collect::<Vec<_>>()
        } else {
            let r = n % 4 + 4;
            let mut ansr = vec![vec!['.'; r]; r];
            let ok = dfs(r, &mut ansr, 0, 0, Some(3));
            assert!(ok);

            (0..n / 4 - 1)
                .flat_map(|i| {
                    (0..4).map(move |j| {
                        itertools::repeat_n('.', i * 4)
                            .chain(ans4[j].citer())
                            .chain(repeat('.'))
                            .take(n)
                            .collect::<Vec<_>>()
                    })
                })
                .chain((0..r).map(|i| {
                    itertools::repeat_n('.', 4 * (n / 4 - 1))
                        .chain(ansr[i].citer())
                        .chain(repeat('.'))
                        .take(n)
                        .collect::<Vec<_>>()
                }))
                .collect::<Vec<_>>()
        }
    };

    for row in ans {
        println!("{}", row.citer().join(""));
    }
}
