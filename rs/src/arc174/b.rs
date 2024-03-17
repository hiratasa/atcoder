use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            a: [i64; 5],
            p: [i64; 5],
        };

        let s = 2 * a[4] + a[3] - a[1] - 2 * a[0];

        let t = p[3];
        let u = p[4];

        // x+2*y+s>=0となるような (x, y) のうち、 f=t*x+u*y が最小になるものを求める
        let ans = if s >= 0 {
            0
        } else {
            // x = max(0, -2*y - s)
            // f = t*(-2*y-s)+u*y
            //   = (-2*t+u)*y - s*t
            let a = -2 * t + u;
            let y0 = if a <= 0 { (-s) / 2 } else { 0 };
            let x0 = max(0, -2 * y0 - s);
            let y1 = ((-s) + 1) / 2;
            let x1 = 0;

            min(t * x0 + u * y0, t * x1 + u * y1)
        };

        println!("{ans}");
    }
}
