use std::cmp::Ordering;

use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        mut r: [usize; n],
        queries: [usize; q],
    };

    r.sort();

    let s = r.into_iter().cumsum::<usize>().collect::<Vec<_>>();

    queries
        .into_iter()
        .map(|x| {
            s.binary_search_by(|&y| y.cmp(&x).then(Ordering::Less))
                .unwrap_err()
        })
        .for_each(|ans| {
            println!("{ans}");
        })
}
