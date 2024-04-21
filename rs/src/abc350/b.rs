use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        t: [Usize1; q],
    };

    let a = t.into_iter().fold(vec![true; n], |mut a, x| {
        a[x] = !a[x];
        a
    });

    let ans = a.into_iter().filter(|&x| x).count();
    println!("{ans}");
}
