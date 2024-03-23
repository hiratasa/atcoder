use std::{cmp::Ordering, iter::once};

use itertools::{repeat_n, Itertools};
use itertools_num::ItertoolsNum;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    let s = s
        .iter()
        .copied()
        .map(|x| x as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let t = t
        .iter()
        .copied()
        .map(|x| x as usize - 'a' as usize)
        .collect::<Vec<_>>();

    let ans = solve(n, &s, &t);
    println!("{ans}");

    // use rand::{rngs::SmallRng, Rng, SeedableRng};
    // let mut rng = SmallRng::seed_from_u64(42);
    // loop {
    //     let n = rng.gen_range(1..5);
    //     let sl = rng.gen_range(1..5);
    //     let s = (0..sl).map(|_| rng.gen_range(0..26)).collect::<Vec<_>>();
    //     let tl = rng.gen_range(1..5);
    //     let t = (0..tl).map(|_| rng.gen_range(0..26)).collect::<Vec<_>>();

    //     assert_eq!(
    //         solve(n, &s, &t),
    //         solve0(n, &s, &t),
    //         "{}\n{}\n{}",
    //         n,
    //         s.iter().map(|&c| (c as u8 + b'a') as char).join(""),
    //         t.iter().map(|&c| (c as u8 + b'a') as char).join("")
    //     );

    //     print!(".");
    // }
}

fn solve(n: usize, s: &[usize], t: &[usize]) -> usize {
    let sl = s.len();

    let nums = (0..26)
        .map(|c| {
            once(0)
                .chain(s.iter().map(|&x| x == c).map(|x| x as usize))
                .cumsum::<usize>()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if t.iter().copied().any(|x| nums[x][sl] == 0) {
        return 0;
    }

    let ans = lower_bound_int(1, 1 << 60, |k: usize| {
        let Some((m, pos)) = t.iter().copied().try_fold((0usize, 0usize), |(m, pos), c| {
            let k2 = k + nums[c][pos];

            let m = m.checked_add((k2 - 1) / nums[c][sl])?;
            let r = k2 - (k2-1) / nums[c][sl] * nums[c][sl];

            let next_pos = nums[c]
                .binary_search_by(|&s| s.cmp(&r).then(Ordering::Greater))
                .unwrap_err();

            let (m, next_pos) = (m + next_pos / sl, next_pos % sl);

            // eprintln!("{}; {}; {} {}", k, c, m, next_pos);

            Some((m, next_pos))
        }) else {
            return Ordering::Greater;
        };

        let m = m + (pos > 0) as usize;

        if m > n {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }) - 1;

    ans
}

fn solve0(n: usize, s: &[usize], t: &[usize]) -> usize {
    let ss = (0..n).map(|_| s).flatten().copied().collect::<Vec<_>>();

    (0..)
        .find(|&i| {
            let ii = i + 1;

            t.iter()
                .flat_map(|c| repeat_n(*c, ii))
                .try_fold(0, |pos, c| {
                    ss[pos..]
                        .iter()
                        .copied()
                        .position(|d| d == c)
                        .map(|q| pos + q + 1)
                })
                .is_none()
        })
        .unwrap()
}

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}
