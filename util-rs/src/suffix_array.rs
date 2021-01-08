use itertools::{iterate, Itertools};

// 0..n のindexを、s[i..]が辞書順になるように並べる
// O(NlogN)
// O(N)のアルゴリズムもある(SA-IS)
#[allow(dead_code)]
fn suffix_array<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let sa0 = (0..n).sorted_by_key(|&i| &s[i]).collect_vec();
    let r0 = sa0
        .iter()
        .group_by(|&&i| &s[i])
        .into_iter()
        .enumerate()
        .fold(vec![0; n], |mut r, (rank, (_, it))| {
            for &idx in it {
                r[idx] = rank;
            }
            r
        });

    iterate(2, |len| len * 2)
        .take_while(|&len| len / 2 < n)
        .fold((sa0, r0), |(_prev_sa, prev_r), len| {
            let to_key = |i: usize| (prev_r.get(i), prev_r.get(i + len / 2));
            let sa = (0..n).sorted_by_key(|&i| to_key(i)).collect_vec();
            let r = sa
                .iter()
                .group_by(|&&i| to_key(i))
                .into_iter()
                .enumerate()
                .fold(vec![0; n], |mut r, (rank, (_, it))| {
                    for &idx in it {
                        r[idx] = rank;
                    }
                    r
                });
            (sa, r)
        })
        .0
}

#[test]
fn test_suffix_array() {
    let s = b"abcdabcaca";
    assert_eq!(suffix_array(s), vec![9, 4, 0, 7, 5, 1, 8, 6, 2, 3]);
}
