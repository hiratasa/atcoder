fn main() {
    input! {
        t: usize,
        cases: [[(i128, i128, i128)]; t],
    };

    for case in cases {
        let ans = solve(case);
        println!("{ans}");
    }

    // let mut rng = SmallRng::seed_from_u64(42);
    // loop {
    //     let n = rng.random_range(1..=10);
    //     let case = (0..n)
    //         .map(|_| {
    //             (
    //                 rng.random_range(1..=10),
    //                 rng.random_range(1..=10),
    //                 rng.random_range(1..=10),
    //             )
    //         })
    //         .collect::<Vec<_>>();

    //     assert_eq!(
    //         solve0(&case),
    //         solve(case.clone()),
    //         "{}\n{}",
    //         n,
    //         case.iter()
    //             .map(|&(a, b, c)| format!("{a} {b} {c}"))
    //             .join("\n")
    //     );

    //     print!(".");
    // }
}

fn solve0(case: &[(i128, i128, i128)]) -> i128 {
    iproduct!(1..=100, 1..=100)
        .filter(|&(x, y)| case.iter().all(|&(a, b, c)| a * x + b * y < c))
        .count() as i128
}

fn solve(mut case: Vec<(i128, i128, i128)>) -> i128 {
    case.sort_by(|&(a0, b0, c0), &(a1, b1, c1)| {
        (a0 * b1).cmp(&(a1 * b0)).then((a1 * c0).cmp(&(a0 * c1)))
    });
    case.dedup_by(|&mut (a0, b0, _c0), &mut (a1, b1, _c1)| a0 * b1 == a1 * b0);

    let is_unnecessary = |lines: &[(i128, i128, i128)], line: (i128, i128, i128)| {
        if lines.len() < 2 {
            false
        } else {
            let (a1, b1, c1) = lines[lines.len() - 2];
            let (a2, b2, c2) = lines[lines.len() - 1];
            let (a3, b3, c3) = line;

            (b1 * c2 - b2 * c1) * (b2 * a3 - b3 * a2) >= (b1 * a2 - b2 * a1) * (b2 * c3 - b3 * c2)
        }
    };

    let lines = case.into_iter().fold(vec![], |mut lines, (a, b, c)| {
        while is_unnecessary(&lines, (a, b, c)) {
            lines.pop();
        }

        lines.push((a, b, c));

        lines
    });

    let m = lines.len();

    // eprintln!("#{lines:?}");

    once((1, 0))
        .chain(lines.iter().copied().tuple_windows().enumerate().map(
            |(i, ((a0, b0, c0), (a1, b1, c1)))| {
                let u = b0 * c1 - b1 * c0;
                let v = b0 * a1 - b1 * a0;
                assert!(v > 0);

                ((u + v - 1).div_euclid(v).max(1), i + 1)
            },
        ))
        .chain(once((
            (lines[m - 1].2 + lines[m - 1].0 - 1) / lines[m - 1].0,
            m,
        )))
        .tuple_windows()
        .map(|((x0, i0), (x1, _i1))| {
            let (a, b, c) = lines[i0];

            let xmax = (c + a - 1) / a;
            if x0 >= xmax {
                return 0;
            }
            let x1 = min(x1, xmax);

            // eprintln!("#[{x0}, {x1}): {:?}", lines[i0]);
            // dbg!(floor_sum(x1 - x0, c - a * x0 - 1, -a, b));

            floor_sum(x1 - x0, c - a * x0 - 1, -a, b)
        })
        .sum::<i128>()
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

// calc sum[x=0 to n-1] floor((a+x*b)/c)
#[allow(dead_code)]
fn floor_sum(n: i128, mut a: i128, mut b: i128, c: i128) -> i128 {
    if n == 0 {
        return 0;
    }

    let mut ret = 0;

    if c < 0 {
        return floor_sum(n, -a, -b, -c);
    }

    ret += a.div_euclid(c) * n;
    a = a.rem_euclid(c);
    assert!(a >= 0);

    ret += b.div_euclid(c) * n * (n - 1) / 2;
    b = b.rem_euclid(c);
    assert!(b >= 0);

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
