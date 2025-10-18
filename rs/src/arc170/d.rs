use std::cmp::{max, min, Ordering};

use proconio::input;

fn solve0(a: &[usize], b: &[usize]) -> bool {
    let n = a.len();

    (0..n).any(|i| {
        (0..n).all(|j| (0..n).any(|k| k != i && a[i].abs_diff(a[k]) < b[j] && b[j] < a[i] + a[k]))
    })
}

fn solve(a: &[usize], b: &[usize]) -> bool {
    let n = a.len();

    let count0 = |lower: usize, upper: usize| {
        let i0 = a
            .binary_search_by(|&x| x.cmp(&lower).then(Ordering::Greater))
            .unwrap_err();
        let i1 = a
            .binary_search_by(|&x| x.cmp(&upper).then(Ordering::Greater))
            .unwrap_err();

        i1 - i0
    };
    let count1 = |lower: usize, upper: usize, exclude: usize| {
        if lower <= exclude && exclude < upper {
            count0(lower, upper) - 1
        } else {
            count0(lower, upper)
        }
    };

    a.iter()
        .copied()
        .rev()
        .filter(|&x| x <= b[0] || count1(x - b[0] + 1, x + b[0], x) > 0)
        .try_fold((n, 0), |(mut i, mut limit), x| {
            if x <= limit {
                return Err(false);
            }

            let mut ok = true;
            while i > 0 && b[i - 1] >= x {
                i -= 1;
                let y = b[i];

                let c = count0(y - x + 1, y + x);

                match c {
                    0 => return Err(false),
                    1 => {
                        if y - x < x && x < y + x {
                            ok = false;
                        }
                    }
                    _ => {}
                }

                let idx = a
                    .binary_search_by(|&z| z.cmp(&y).then(Ordering::Greater))
                    .unwrap_err();
                let nearest = min(
                    a.get(idx).copied().unwrap_or(usize::MAX),
                    idx.checked_sub(1).map_or(usize::MAX, |ii| a[ii]),
                );
                limit = max(limit, y.abs_diff(nearest));

                // eprintln!("x={x}; i={i}, limit={limit}, c={c}, ok={ok}");
            }

            if ok {
                Err(true)
            } else {
                Ok((i, limit))
            }
        })
        .map_or_else(|e| e, |_| false)
}

fn main() {
    input! {
        t: usize,
    };

    // let t = usize::MAX;
    // use itertools::Itertools;
    // use rand::{rngs::SmallRng, Rng, SeedableRng};
    // use std::iter::*;
    // let mut rng = SmallRng::seed_from_u64(42);

    for _ in 0..t {
        input! {
            n: usize,
            mut a: [usize; n],
            mut b: [usize; n],
        };
        // let n = rng.random_range(2..20);
        // let mut a = repeat_with(|| rng.random_range(1..100))
        //     .take(n)
        //     .collect::<Vec<_>>();
        // let mut b = repeat_with(|| rng.random_range(1..100))
        //     .take(n)
        //     .collect::<Vec<_>>();

        a.sort();
        b.sort();

        let ans = solve(&a, &b);

        if ans {
            println!("Alice");
        } else {
            println!("Bob");
        }

        // let ans0 = solve0(&a, &b);
        // assert_eq!(
        //     ans,
        //     ans0,
        //     "{}\n{}\n{}",
        //     n,
        //     a.iter().join(" "),
        //     b.iter().join(" ")
        // );
    }
}
