use std::cmp::min;

use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    };

    let t = ac
        .into_iter()
        .fold(FxHashMap::default(), |mut map, (a, c)| {
            let b = map.entry(c).or_insert(usize::MAX);

            *b = min(*b, a);

            map
        });

    println!("{}", t.values().copied().max().unwrap());
}
