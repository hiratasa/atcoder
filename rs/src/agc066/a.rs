use itertools::{Itertools, iproduct, izip};
use proconio::input;

fn main() {
    input! {
        n: usize, d: i64,
        a:[[i64; n]; n],
    };

    let even_to_even = iproduct!(0..n, 0..n)
        .filter(|&(i, j)| (i + j) % 2 == 0)
        .map(|(i, j)| a[i][j])
        .map(|x| {
            if x.div_euclid(d) % 2 == 0 {
                x.rem_euclid(d)
            } else {
                d - x.rem_euclid(d)
            }
        })
        .sum::<i64>();
    let odd_to_even = iproduct!(0..n, 0..n)
        .filter(|&(i, j)| (i + j) % 2 > 0)
        .map(|(i, j)| a[i][j])
        .map(|x| {
            if x.div_euclid(d) % 2 == 0 {
                x.rem_euclid(d)
            } else {
                d - x.rem_euclid(d)
            }
        })
        .sum::<i64>();

    let num_even = iproduct!(0..n, 0..n)
        .filter(|&(i, j)| (i + j) % 2 == 0)
        .count() as i64;
    let num_odd = iproduct!(0..n, 0..n)
        .filter(|&(i, j)| (i + j) % 2 > 0)
        .count() as i64;

    let cost0 = even_to_even + d * num_odd - odd_to_even;
    let cost1 = odd_to_even + d * num_even - even_to_even;

    let mut b = a.clone();

    for i in 0..n {
        for j in 0..n {
            let x = a[i][j];
            b[i][j] -= x.rem_euclid(d);
            if ((b[i][j].div_euclid(d) % 2 != 0) == ((i + j) % 2 == 0)) == (cost0 <= cost1) {
                b[i][j] += d;
            }
        }
    }

    let cost = izip!(a.iter().flatten(), b.iter().flatten())
        .map(|(&x, &y)| x.abs_diff(y))
        .sum::<u64>();
    eprintln!("cost={cost}");

    for row in b {
        println!("{}", row.iter().join(" "));
    }
}
