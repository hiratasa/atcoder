use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        x: [Usize1; q],
    };

    let (mut a, b, _, s) = x.into_iter().fold(
        (vec![0; n], vec![None; n], 0usize, 0usize),
        |(mut a, mut b, size, s), x| {
            let size = if let Some(u) = b[x] {
                a[x] += s - u;
                b[x] = None;
                size - 1
            } else {
                b[x] = Some(s);
                size + 1
            };

            (a, b, size, s + size)
        },
    );

    for (i, u) in b.into_iter().enumerate() {
        if let Some(u) = u {
            a[i] += s - u;
        }
    }

    println!("{}", a.iter().join(" "));
}
