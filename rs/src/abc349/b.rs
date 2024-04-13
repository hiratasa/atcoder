use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };

    if s.into_iter()
        .fold(vec![0; 26], |mut freq, c| {
            freq[c as usize - 'a' as usize] += 1;
            freq
        })
        .into_iter()
        .filter(|&x| x > 0)
        .sorted()
        .dedup_with_count()
        .into_iter()
        .all(|(num, _)| num == 2)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
