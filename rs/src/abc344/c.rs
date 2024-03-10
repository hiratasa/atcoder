use itertools::iproduct;
use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        a: [usize],
        b: [usize],
        c: [usize],
        x: [usize],
    };

    let sums = iproduct!(a, b, c)
        .map(|(x, y, z)| x + y + z)
        .collect::<FxHashSet<_>>();

    x.into_iter().map(|xx| sums.contains(&xx)).for_each(|ans| {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    });
}
