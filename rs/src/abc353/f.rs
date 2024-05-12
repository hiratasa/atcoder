use std::cmp::{max, min};

use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        k: usize,
        sx: usize, sy: usize,
        tx: usize, ty: usize,
    };

    let ans0 = if (sx / k, sy / k) == (tx / k, ty / k) {
        if (sx / k + sy / k) % 2 == 0 {
            sx.abs_diff(tx) + sy.abs_diff(ty)
        } else {
            0
        }
    } else {
        usize::MAX
    };

    let (sx, sy) = (sx + k, sy + k);
    let (tx, ty) = (tx + k, ty + k);

    let get_big_tiles = |x: usize, y: usize| {
        if (x / k + y / k) % 2 == 0 {
            vec![
                (x / k - 1, y / k, x % k + 1),
                (x / k + 1, y / k, k - x % k),
                (x / k, y / k - 1, y % k + 1),
                (x / k, y / k + 1, k - y % k),
            ]
        } else {
            vec![(x / k, y / k, 0)]
        }
    };

    let ans1 = iproduct!(get_big_tiles(sx, sy), get_big_tiles(tx, ty))
        .map(|((ax, ay, c0), (bx, by, c1))| {
            let c = c0 + c1;

            let dx = ax.abs_diff(bx);
            let dy = ay.abs_diff(by);

            let mi = min(dx, dy);
            let d = max(dx, dy) - mi;

            mi * 2 + (d / 2) * min(4, k + 1) + c
        })
        .min()
        .unwrap();

    let ans = min(ans0, ans1);

    println!("{ans}");
}
