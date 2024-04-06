use bitset_fixed::BitSet;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [[usize; m]; n],
    };

    let bs = (0..m)
        .map(|i| {
            a.iter().map(|aa| aa[i]).enumerate().fold(
                vec![BitSet::new(n); 1000],
                |mut bs, (j, x)| {
                    bs[x].set(j, true);
                    bs
                },
            )
        })
        .collect::<Vec<_>>();

    let ans = (0..n)
        .map(|i| {
            let mut similar = (0..m).fold(BitSet::new(n), |similar, j| {
                let x = a[i][j];
                let p = &bs[j][x];

                similar ^ p
            });

            similar.set(i, false);

            similar.count_ones() as usize
        })
        .sum::<usize>()
        / 2;

    println!("{ans}");
}
