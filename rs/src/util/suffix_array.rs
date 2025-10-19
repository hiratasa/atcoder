use itertools::{Itertools, iterate};
use itertools_num::ItertoolsNum;

// 0..n のindexを、s[i..]が辞書順になるように並べる
// O(NlogN)
// 定数倍が遅そう（各stepのsa構築時のランダムアクセスがきついっぽい）
// O(N)のアルゴリズムもある(SA-IS)
#[allow(dead_code)]
fn suffix_array<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    // 同じ文字間では添え字の逆順に並べる
    let sa0 = (0..n)
        .sorted_by_key(|&i| (&s[i], std::cmp::Reverse(i)))
        .collect_vec();
    let (rank0, max_rank) = sa0
        .chunk_by(|&i, &j| s[i] == s[j])
        .enumerate()
        .fold((vec![0; n], 0), |(mut rank, _), (r, chunk)| {
            for &idx in chunk {
                rank[idx] = r;
            }
            (rank, r)
        });

    iterate(2, |len| len * 2)
        .take_while(|&len| len / 2 < n)
        .try_fold(
            (sa0, rank0, max_rank),
            |(prev_sa, prev_rank, prev_max_rank), len| {
                let counts =
                    prev_rank
                        .iter()
                        .fold(vec![0; prev_max_rank + 1], |mut counts, &idx| {
                            counts[idx] += 1;
                            counts
                        });
                let cum_counts = counts.iter().cumsum::<usize>().collect::<Vec<_>>();

                // prev_saは各suffixのlen/2文字の部分の昇順になっており、
                // かつlen/2文字の部分が同じときは添え字の降順に並んでいる
                // => n-len/2より大きいものはprev_saから変化なし
                //    それ以外の部分は前半len/2文字分で安定ソートする
                let sa = prev_sa
                    .iter()
                    .copied()
                    .filter_map(|i| i.checked_sub(len / 2))
                    .rev()
                    .fold(
                        (prev_sa.clone(), cum_counts),
                        |(mut sa, mut cum_counts), i| {
                            cum_counts[prev_rank[i]] -= 1;
                            sa[cum_counts[prev_rank[i]]] = i;
                            (sa, cum_counts)
                        },
                    )
                    .0;

                let to_key = |i: usize| (prev_rank.get(i), prev_rank.get(i + len / 2));
                let (rank, max_rank) = sa
                    .chunk_by(|&i, &j| to_key(i) == to_key(j))
                    .enumerate()
                    .fold((vec![0; n], 0), |(mut rank, _), (r, chunk)| {
                        for &idx in chunk {
                            rank[idx] = r;
                        }
                        (rank, r)
                    });

                if max_rank == n - 1 {
                    // これ以上の比較は不要
                    Err((sa, rank))
                } else {
                    Ok((sa, rank, max_rank))
                }
            },
        )
        // n=1のときはerrにならないので注意
        .map_or_else(|(sa, _rank)| sa, |(sa, _rank, _)| sa)
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
    let cases = [
        (b"" as &[u8], vec![]),
        (b"a", vec![0]),
        (b"aaaaaa", vec![5, 4, 3, 2, 1, 0]),
        (b"abababab", vec![6, 4, 2, 0, 7, 5, 3, 1]),
        (b"abcdabcaca", vec![9, 4, 0, 7, 5, 1, 8, 6, 2, 3]),
    ];

    for (s, sa) in cases {
        assert_eq!(suffix_array(s), sa, "s = {s:?}");
    }
}

#[test]
fn test_suffix_array_random() {
    for _ in 0..100 {
        let n = 1000;

        let s = (0..n).map(|_| rand::random::<u8>()).collect::<Vec<_>>();

        let expected = (0..n).sorted_by_key(|&i| &s[i..]).collect::<Vec<_>>();

        assert_eq!(suffix_array(&s), expected, "s = {s:?}");
    }
}

#[test]
fn measure_suffix_array_large() {
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    let mut rng = SmallRng::seed_from_u64(42);

    let n = 100000;

    let s = (0..n)
        .map(|_| rng.random_range(0..1 << 8))
        .collect::<Vec<_>>();

    for _ in 0..100 {
        let _ = suffix_array(&s);
    }
}
