use std::cmp::{max, min};

use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        v1: i64, v2: i64, v3: i64,
    };

    if v1 + 2 * v2 + 3 * v3 != 3 * 7i64.pow(3) {
        println!("No");
        return;
    };

    const L: i64 = 7;

    if let Some(ans) = (0..3)
        .map(|_| 0..3)
        .multi_cartesian_product()
        .find_map(|zero_idxs| {
            let mut dims0 = [[None; 3]; 3];
            for (i, idx) in zero_idxs.into_iter().enumerate() {
                dims0[idx][i] = Some(0);
            }

            iproduct!(0..3, 0..3)
                .map(|(i, j)| {
                    if let Some(d) = dims0[i][j] {
                        d..=d
                    } else {
                        0..=14
                    }
                })
                .multi_cartesian_product()
                .find(|dims| {
                    let u3 = (0..3)
                        .map(|i| {
                            let mi = (0..3).map(|j| dims[3 * j + i]).max().unwrap();
                            let ma = (0..3).map(|j| dims[3 * j + i] + L).min().unwrap();

                            (ma - mi).max(0)
                        })
                        .product::<i64>();

                    let u2 = (0..3)
                        .tuple_combinations()
                        .map(|(idx0, idx1)| {
                            (0..3)
                                .map(|i| {
                                    let mi = max(dims[3 * idx0 + i], dims[3 * idx1 + i]);
                                    let ma = min(dims[3 * idx0 + i], dims[3 * idx1 + i]) + L;

                                    (ma - mi).max(0)
                                })
                                .product::<i64>()
                        })
                        .sum::<i64>()
                        - 3 * u3;

                    u2 == v2 && u3 == v3
                })
        })
    {
        println!("Yes");
        println!("{}", ans.into_iter().join(" "));
    } else {
        println!("No");
    }
}
