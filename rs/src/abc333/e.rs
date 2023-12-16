use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        tx: [(usize, Usize1); n],
    };

    let Some((d, _)) = tx.iter().copied().enumerate().try_fold(
        (vec![0i64; n], vec![vec![]; n]),
        |(mut d, mut stacks), (i, (t, x))| {
            if t == 1 {
                stacks[x].push(i);

                Some((d, stacks))
            } else {
                let i0 = stacks[x].pop()?;

                d[i0] += 1;
                d[i] -= 1;

                Some((d, stacks))
            }
        },
    ) else {
        println!("-1");
        return;
    };

    let ans = d.iter().copied().cumsum::<i64>().max().unwrap();

    println!("{ans}");
    println!(
        "{}",
        tx.iter()
            .copied()
            .positions(|(t, _)| t == 1)
            .map(|i| d[i])
            .join(" ")
    );
}
