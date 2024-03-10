use std::iter::repeat_with;

use proconio::input;

fn main() {
    let v = repeat_with(|| {
        input! { a: usize };

        a
    })
    .take_while(|&a| a > 0)
    .collect::<Vec<_>>();

    println!("0");
    for x in v.into_iter().rev() {
        println!("{x}");
    }
}
