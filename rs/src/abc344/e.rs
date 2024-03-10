use std::iter::successors;

use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    };

    a.insert(0, 0);
    a.push(usize::MAX);
    let mut prevs = a
        .iter()
        .copied()
        .tuple_windows()
        .map(|(x, y)| (y, x))
        .collect::<FxHashMap<_, _>>();
    let mut nexts = a
        .iter()
        .copied()
        .tuple_windows()
        .collect::<FxHashMap<_, _>>();

    for _ in 0..q {
        input! { ty: usize };

        if ty == 1 {
            input! { x: usize, y: usize, };

            let z = nexts[&x];
            prevs.insert(z, y);
            prevs.insert(y, x);
            nexts.insert(x, y);
            nexts.insert(y, z);
        } else {
            input! { x: usize };

            let prev = prevs[&x];
            let next = nexts[&x];

            prevs.insert(next, prev);
            nexts.insert(prev, next);

            prevs.remove(&x);
            nexts.remove(&x);
        }
    }

    println!(
        "{}",
        successors(Some(0), |&x| nexts.get(&x).copied())
            .filter(|&x| x > 0 && x < usize::MAX)
            .join(" ")
    );
}
