use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };

    let n = s.len();

    let freq = s.iter().fold(vec![0usize; 26], |mut freq, &c| {
        freq[c as usize - 'a' as usize] += 1;
        freq
    });

    let ans = if freq.iter().all(|&x| x <= 1) {
        n * (n - 1) / 2
    } else {
        n * (n - 1) / 2 + 1
            - freq
                .iter()
                .map(|&x| x * x.saturating_sub(1) / 2)
                .sum::<usize>()
    };

    println!("{ans}");
}
