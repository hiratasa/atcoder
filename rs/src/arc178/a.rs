use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [usize; m],
    };

    if a.contains(&1) {
        println!("-1");
        return;
    }

    let t = a.iter().fold(vec![false; 2 * n + 10], |mut t, x| {
        t[*x] = true;
        t
    });

    let ans = (1..=n).try_fold(
        (vec![], VecDeque::new(), 0),
        |(mut v, mut q, last): (Vec<usize>, VecDeque<usize>, usize), _| {
            if q.len() > 1 {
                v.push(q.pop_front().unwrap());
                Some((v, q, last))
            } else if matches!(q.front(), Some(_) if !t[last]) {
                v.push(q.pop_front().unwrap());
                Some((v, q, last))
            } else if q.is_empty() {
                if t[last + 1] {
                    q.push_back(last + 1);
                    v.push(last + 2);

                    Some((v, q, last + 2))
                } else {
                    v.push(last + 1);

                    Some((v, q, last + 1))
                }
            } else {
                v.push(last + 1);

                Some((v, q, last + 1))
            }
        },
    );

    if let Some((ans, _, last)) = ans {
        if last > n {
            println!("-1");
        } else {
            println!("{}", ans.iter().join(" "));
        }
    } else {
        println!("-1");
    }
}
