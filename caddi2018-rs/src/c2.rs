#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_macros)]
macro_rules! read_cols {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn calc<I: std::iter::Iterator<Item = usize> + Clone>(
    iter: I,
) -> impl std::iter::Iterator<Item = usize> {
    std::iter::once(0).chain(
        iter.clone()
            .zip(iter.clone().skip(1))
            .map(|(prev, a)| {
                if prev < a {
                    -((((a + prev - 1) / prev).next_power_of_two().trailing_zeros() as i64 + 1) / 2
                        * 2)
                } else {
                    ((prev / a + 1).next_power_of_two().trailing_zeros() as i64 - 1) / 2 * 2
                }
            })
            .scan(vec![(usize::MAX, 1)], |caps, b| {
                if b > 0 {
                    caps.push((b as usize, 1));
                    Some(0)
                } else {
                    let mut t = 0;
                    let mut c = (-b) as usize;
                    while c >= caps.last().unwrap().0 {
                        c -= caps.last().unwrap().0;
                        t += caps.last().unwrap().0 * caps.last().unwrap().1;

                        let tmp = caps.last().unwrap().1;
                        caps.pop();
                        caps.last_mut().unwrap().1 += tmp;
                    }

                    t += c * caps.last().unwrap().1;
                    caps.last_mut().unwrap().0 -= c;
                    caps.last_mut().unwrap().1 += 1;
                    Some(t)
                }
            })
            .scan(0, |acc, c| {
                *acc += c;
                Some(*acc)
            }),
    )
}

fn main() {
    let _: usize = read();
    let a = read_vec::<usize>();

    let b1 = std::iter::once(0)
        .chain(calc(a.iter().copied()).enumerate().map(|(i, v)| v + i + 1))
        .collect::<Vec<_>>();
    let b2 = std::iter::once(0)
        .chain(calc(a.iter().copied().rev()))
        .collect::<Vec<_>>();

    let ans = b1
        .iter()
        .zip(b2.iter().rev())
        .map(|(v1, v2)| v1 + v2)
        .min()
        .unwrap();

    println!("{}", ans);
}
