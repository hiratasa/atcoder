use itertools::{iterate, Itertools};
use itertools_num::ItertoolsNum;

// 0..n のindexを、s[i..]が辞書順になるように並べる
// O(NlogN)
// 定数倍が遅そう（各stepのsa構築時のランダムアクセスがきついっぽい）
// group_byも少し重そうなのでぎりぎりのときはgroup_byをべた書きしてみる
// O(N)のアルゴリズムもある(SA-IS)
#[allow(dead_code)]
fn suffix_array<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let sa0 = (0..n).sorted_by_key(|&i| &s[i]).collect_vec();
    let rank0 = sa0
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
        .fold((sa0, rank0), |(prev_sa, prev_rank), len| {
            let num_rank = prev_rank.iter().max().unwrap() + 1;

            if num_rank == n {
                // これ以上の比較は不要
                return (prev_sa, prev_rank);
            }

            let counts = prev_rank
                .iter()
                .fold(vec![0; num_rank], |mut counts, &idx| {
                    counts[idx] += 1;
                    counts
                });
            let cum_counts = counts.iter().cumsum::<usize>().collect::<Vec<_>>();

            // (prev_rank.get(i), prev_rank.get(i + len/2)) でソートした配列を計算する
            // (n-len/2..n).chain(prev_sa.iter().copied().filter_map(|i| i.checked_sub(len/2))) は2つ目の要素でソートされた状態
            // それをカウントソートしていく
            let sa = (n - len / 2..n)
                .chain(
                    prev_sa
                        .iter()
                        .copied()
                        .filter_map(|i| i.checked_sub(len / 2)),
                )
                .rev()
                .fold((vec![0; n], cum_counts), |(mut sa, mut cum_counts), i| {
                    cum_counts[prev_rank[i]] -= 1;
                    sa[cum_counts[prev_rank[i]]] = i;
                    (sa, cum_counts)
                })
                .0;

            let to_key = |i: usize| (prev_rank.get(i), prev_rank.get(i + len / 2));
            let rank = sa
                .iter()
                .group_by(|&&i| to_key(i))
                .into_iter()
                .enumerate()
                .fold(vec![0; n], |mut rank, (r, (_, it))| {
                    for &idx in it {
                        rank[idx] = r;
                    }
                    rank
                });
            (sa, rank)
        })
        .0
}

#[allow(dead_code)]
fn lcp_array(s: &[char], sa: &[usize], sa_rank: &[usize]) -> Vec<usize> {
    let n = sa_rank.len();

    let mut lcp = vec![0; n - 1];

    let mut l = 0;
    for i in 0..n {
        if sa_rank[i] == 0 {
            continue;
        }

        let i1 = i;
        let i2 = sa[sa_rank[i] - 1];
        while i1 + l < n && i2 + l < n && s[i1 + l] == s[i2 + l] {
            l += 1;
        }

        lcp[sa_rank[i] - 1] = l;
        l = l.checked_sub(1).unwrap_or(0);
    }

    lcp
}

#[test]
fn test_suffix_array() {
    let s = b"abcdabcaca";
    assert_eq!(suffix_array(s), vec![9, 4, 0, 7, 5, 1, 8, 6, 2, 3]);
}
