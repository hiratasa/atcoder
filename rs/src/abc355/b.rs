use itertools::{Itertools, chain};
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    };

    if chain(a.into_iter().map(|x| (x, 0)), b.into_iter().map(|x| (x, 1)))
        .sorted()
        .tuple_windows()
        .any(|((_, x), (_, y))| x == 0 && y == 0)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
