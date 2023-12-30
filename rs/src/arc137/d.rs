use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    };

    const L: usize = 20;

    let mut t = a
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; 1 << L], |mut t, (i, x)| {
            t[(1 << L) - 1 - (n - i - 1)] ^= x;
            t
        });

    for i in 0..L {
        for j in 0..1 << L {
            if j & (1 << i) == 0 {
                t[j] = t[j] ^ t[j ^ (1 << i)];
            }
        }
    }

    println!("{}", (1..=m).map(|k| t[k - 1]).join(" "));
}
