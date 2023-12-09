use proconio::input;

fn main() {
    input! {
        n: usize, s: usize,
        a: [usize; n],
    };

    let ans = a
        .iter()
        .copied()
        .enumerate()
        .scan((0, 0, vec![1; n]), |(j, sum, coeff), (i, x)| {
            while *j < n && *sum <= s {
                *sum += a[*j];
                *j += 1;
            }

            if *sum > s {
                assert!(*j > i + 1);

                coeff[*j - 1] += coeff[i];
            } else {
                assert!(*j == n);
            }

            *sum -= x;

            Some(coeff[i] * (n - i))
        })
        .sum::<usize>();

    println!("{ans}");
}
