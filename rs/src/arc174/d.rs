use std::{cmp::min, mem::replace};

use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! { n: usize };

        // type0: y=10^k, x=10^(2k) + z (0<=z<10^k) (k>=0)
        let ans0 = (0..)
            .scan(1usize, |x, _| Some(replace(x, *x * 10)))
            .take_while(|&x| x.saturating_mul(x) <= n)
            .map(|y| {
                let x0 = y * y;
                let x1 = min(y * y + (y - 1), n);

                x1 - x0 + 1
            })
            .sum::<usize>();

        // type1: y=10^k-1, x=10^(2k)-10^k + z (0<=z<10^k) (k>0)
        let ans1 = (0..)
            .scan(1usize, |x, _| Some(replace(x, *x * 10)))
            .skip(1)
            .take_while(|&x| x.saturating_mul(x - 1) <= n)
            .map(|y| {
                let x0 = y * y - y;
                let x1 = min(y * y - y + y - 1, n);

                x1 - x0 + 1
            })
            .sum::<usize>();

        // type2: y=10^k-2, x=10^(2k)-2*10^k (z<10^k) (k>0)
        let ans2 = (0..)
            .scan(1usize, |x, _| Some(replace(x, *x * 10)))
            .skip(1)
            .take_while(|&x| x.saturating_mul(x - 2) <= n)
            .count();

        println!("{}", ans0 + ans1 + ans2);
    }
}
