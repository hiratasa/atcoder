use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    };

    let ans = (0..=1000000)
        .filter_map(|i| {
            let c = izip!(q.iter().copied(), a.iter().copied())
                .map(|(qq, aa)| qq.checked_sub(aa * i))
                .collect::<Option<Vec<_>>>()?;

            let j = izip!(c.iter().copied(), b.iter().copied())
                .map(|(cc, bb)| if bb == 0 { usize::MAX } else { cc / bb })
                .min()
                .unwrap();

            Some(i + j)
        })
        .max()
        .unwrap();

    println!("{ans}");
}
