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

fn main() {
    let n: usize = read();
    let s = read_row::<usize>();
    let t = read_row::<usize>();
    let u = read_row::<usize>();
    let v = read_row::<usize>();

    let ans = (0..64)
        .map(|pos| {
            let table = vec![vec![None; n]; n];

            let (table, row_idxs) = (0..n).fold((table, vec![]), |(mut table, mut idxs), i| {
                if s[i] == 0 {
                    // bit-and
                    if (u[i] >> pos) & 1 > 0 {
                        table[i].iter_mut().for_each(|x| *x = Some(true));
                    } else {
                        idxs.push((i, false));
                    }
                } else {
                    // bit-or
                    if (u[i] >> pos) & 1 > 0 {
                        idxs.push((i, true));
                    } else {
                        table[i].iter_mut().for_each(|x| *x = Some(false));
                    }
                }

                (table, idxs)
            });

            let (table, col_idxs) =
                (0..n).try_fold((table, vec![]), |(mut table, mut idxs), i| {
                    if t[i] == 0 {
                        // bit-and
                        if (v[i] >> pos) & 1 > 0 {
                            if !(0..n).all(|j| {
                                if table[j][i] == Some(false) {
                                    false
                                } else {
                                    table[j][i] = Some(true);
                                    true
                                }
                            }) {
                                return None;
                            }
                        } else {
                            idxs.push((i, false));
                        }
                    } else {
                        // bit-or
                        if (v[i] >> pos) & 1 > 0 {
                            idxs.push((i, true));
                        } else {
                            if !(0..n).all(|j| {
                                if table[j][i] == Some(true) {
                                    false
                                } else {
                                    table[j][i] = Some(false);
                                    true
                                }
                            }) {
                                return None;
                            }
                        }
                    }

                    Some((table, idxs))
                })?;

            if row_idxs.is_empty() || col_idxs.is_empty() {
                let is_ok = (0..n).all(|i| {
                    if s[i] == 0 {
                        (0..n).all(|j| table[i][j].unwrap()) == (((u[i] >> pos) & 1) > 0)
                    } else {
                        (0..n).any(|j| table[i][j].unwrap()) == (((u[i] >> pos) & 1) > 0)
                    }
                }) && (0..n).all(|i| {
                    if t[i] == 0 {
                        (0..n).all(|j| table[j][i].unwrap()) == (((v[i] >> pos) & 1) > 0)
                    } else {
                        (0..n).any(|j| table[j][i].unwrap()) == (((v[i] >> pos) & 1) > 0)
                    }
                });

                if is_ok {
                    return Some(table);
                } else {
                    return None;
                }
            }

            let apply_row = |table: Vec<Vec<Option<bool>>>| {
                row_idxs.citer().fold(table, |mut table, (row_idx, x)| {
                    table[row_idx]
                        .iter_mut()
                        .for_each(|y| *y = Some(y.unwrap_or(x)));
                    table
                })
            };

            let apply_col = |table: Vec<Vec<Option<bool>>>| {
                col_idxs.citer().fold(table, |mut table, (col_idx, x)| {
                    (0..n).for_each(|j| {
                        table[j][col_idx] = Some(table[j][col_idx].unwrap_or(x));
                    });
                    table
                })
            };

            if let Some((row_idx, _)) = row_idxs
                .citer()
                .find(|&(i, x)| table[i].citer().any(|y| y == Some(x)))
            {
                let table = col_idxs.citer().fold(table, |mut table, (col_idx, x)| {
                    table[row_idx][col_idx] = Some(x);
                    table
                });

                let table = apply_row(table);
                let table = apply_col(table);
                return Some(table);
            }

            if let Some((col_idx, _)) = col_idxs
                .citer()
                .find(|&(i, x)| (0..n).any(|j| table[j][i] == Some(x)))
            {
                let table = row_idxs.citer().fold(table, |mut table, (row_idx, x)| {
                    table[row_idx][col_idx] = Some(x);
                    table
                });

                let table = apply_col(table);
                let table = apply_row(table);
                return Some(table);
            }

            if row_idxs.citer().any(|t| t.1) && row_idxs.citer().any(|t| !t.1) {
                let table = apply_row(table);
                let table = apply_col(table);
                return Some(table);
            }

            if col_idxs.citer().any(|t| t.1) && col_idxs.citer().any(|t| !t.1) {
                let table = apply_col(table);
                let table = apply_row(table);

                return Some(table);
            }

            if row_idxs[0].1 == col_idxs[0].1 {
                let table = apply_row(table);
                let table = apply_col(table);

                return Some(table);
            }

            if row_idxs.len() == 1 || col_idxs.len() == 1 {
                return None;
            }

            let table = (0..max(row_idxs.len(), col_idxs.len())).fold(table, |mut table, i| {
                table[row_idxs[i % row_idxs.len()].0][col_idxs[i % col_idxs.len()].0] = Some(true);
                table
            });

            // fill zero
            let table = (0..n).fold(table, |mut table, i| {
                table[i]
                    .iter_mut()
                    .for_each(|x| *x = Some(x.unwrap_or(false)));
                table
            });

            Some(table)
        })
        .enumerate()
        .try_fold(vec![vec![0usize; n]; n], |mut ans, (pos, table)| {
            let table = table?;

            for i in 0..n {
                for j in 0..n {
                    if table[i][j].unwrap() {
                        ans[i][j] |= 1 << pos;
                    }
                }
            }

            Some(ans)
        });

    if let Some(ans) = ans {
        for row in ans {
            println!("{}", row.citer().join(" "));
        }
    } else {
        println!("-1");
    }
}
