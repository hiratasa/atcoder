use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let lasts = a
        .iter()
        .copied()
        .enumerate()
        .scan([None; 11], |lasts, (i, x)| {
            let l = *lasts;
            lasts[x] = Some(i);

            Some(l)
        })
        .collect::<Vec<_>>();

    let t = a
        .iter()
        .copied()
        .enumerate()
        .map(|(i, x)| {
            (1..=10)
                .filter(|&y| (x + y) % 2 == 0)
                .filter_map(|y| {
                    let z = (x + y) / 2;

                    let j = lasts[i][z]?;
                    let k = lasts[j][y]?;

                    Some(k)
                })
                .max()
        })
        .collect::<Vec<_>>();

    let ans = (0..n)
        .scan(0, |j, i| {
            while *j < n && !matches!(t[*j], Some(k) if k >= i) {
                *j += 1;
            }

            if *j == n {
                return None;
            }

            Some(n - *j)
        })
        .sum::<usize>();

    println!("{ans}");
}
