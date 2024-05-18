use std::cmp::Ordering;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        };

        let t = a
            .iter()
            .copied()
            .scan(vec![], |t: &mut Vec<usize>, x| {
                let idx = t
                    .binary_search_by(|&y| y.cmp(&x).then(Ordering::Greater))
                    .unwrap_err();

                if idx == t.len() {
                    t.push(x);
                } else {
                    t[idx] = x;
                }

                Some(idx + 1)
            })
            .collect::<Vec<_>>();

        let u = a
            .iter()
            .copied()
            .rev()
            .scan(vec![], |t: &mut Vec<usize>, x| {
                let idx = t
                    .binary_search_by(|&y| y.cmp(&x).reverse().then(Ordering::Greater))
                    .unwrap_err();

                if idx == t.len() {
                    t.push(x);
                } else {
                    t[idx] = x;
                }

                Some(idx + 1)
            })
            .collect::<Vec<_>>();

        let l = t.iter().copied().max().unwrap();

        let ans = (0..n)
            .filter(|&i| t[i] + u[n - 1 - i] == l + 1)
            .collect::<Vec<_>>();
        println!("{}", ans.len());
        println!("{}", ans.iter().map(|i| i + 1).join(" "));
    }
}
