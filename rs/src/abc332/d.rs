use itertools::{Itertools, iproduct};
use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    };

    let ans = iproduct!((0..h).permutations(h), (0..w).permutations(w))
        .filter(|(i_perm, j_perm)| {
            iproduct!(i_perm.iter().copied(), j_perm.iter().copied())
                .map(|(i, j)| a[i][j])
                .eq(b.iter().flatten().copied())
        })
        .map(|(i_perm, j_perm)| {
            i_perm
                .into_iter()
                .tuple_combinations()
                .filter(|&(i0, i1)| i0 > i1)
                .count()
                + j_perm
                    .into_iter()
                    .tuple_combinations()
                    .filter(|&(i0, i1)| i0 > i1)
                    .count()
        })
        .min();

    if let Some(ans) = ans {
        println!("{ans}");
    } else {
        println!("-1");
    }
}
