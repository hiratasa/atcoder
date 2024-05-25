use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    };

    lr.sort();

    let ans = lr
        .into_iter()
        .scan(
            BinaryHeap::new(),
            |q: &mut BinaryHeap<Reverse<usize>>, (l, r)| {
                while matches!(q.peek(), Some(&Reverse(rr)) if rr < l) {
                    q.pop();
                }

                let x = q.len();

                q.push(Reverse(r));

                Some(x)
            },
        )
        .sum::<usize>();

    println!("{ans}");
}
