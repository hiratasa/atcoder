// https://www.geeksforgeeks.org/z-algorithm-linear-time-pattern-searching-algorithm/
// Z-Arrayの構築
// O(N)
// for i < n, z[i] is max z[i] < n such that s[0:z[i]] = s[i:i+z[i]]
// パターンの検索にも使える
#[allow(dead_code)]
fn z_algorithm<T: std::cmp::Eq>(s: &Vec<T>) -> Vec<usize> {
    let n = s.len();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    let mut z = vec![0; n];
    z[0] = n;

    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        // assert!(s[l..r] == s[0..r - l]);
        if i < r && z[i - l] < r - i {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i..r] (=s[i-l..r-l]) = s[0..r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = std::cmp::max(i, r);
            while r < n && s[r] == s[r - l] {
                r += 1;
            }
            z[i] = r - l;
        }
    }

    z
}

// textからpatternの出現箇所を全部検索
#[allow(dead_code)]
fn find_all<T: Eq>(text: &Vec<T>, pattern: &Vec<T>) -> Vec<usize> {
    // pattern + text
    let s = pattern.iter().chain(text.iter()).collect::<Vec<_>>();

    let z = z_algorithm(&s);

    (0..text.len())
        .filter(|&i| z[pattern.len() + i] >= pattern.len())
        .collect()
}

#[test]
fn test_find_all() {
    let s = "ABC ABCDAB ABCDABCDABDEABCDABDFF"
        .chars()
        .collect::<Vec<_>>();
    let w = "ABCDABD".chars().collect::<Vec<_>>();

    assert_eq!(find_all(&s, &w), vec![15, 23]);
}
