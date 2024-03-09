use itertools::iterate;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let ans = (1usize..)
        .map(|x| x.pow(3))
        .take_while(|&x| x <= n)
        .filter(|&x| {
            let y = iterate(x, |&y| y / 10)
                .take_while(|&y| y > 0)
                .map(|y| y % 10)
                .fold(0, |y, z| y * 10 + z);

            y == x
        })
        .last()
        .unwrap();

    println!("{ans}");
}
