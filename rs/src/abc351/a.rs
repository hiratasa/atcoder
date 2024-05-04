use proconio::input;

fn main() {
    input! {
        a: [usize; 9],
        b: [usize; 8],
    };

    let ans = a.into_iter().sum::<usize>() - b.into_iter().sum::<usize>() + 1;

    println!("{ans}");
}
