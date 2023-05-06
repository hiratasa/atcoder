use cargo_snippet::snippet;

const M: usize = 1000000007;

#[snippet("fact")]
#[allow(dead_code)]
fn generate_fact(n: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let fact: Vec<_> = std::iter::once(1)
        .chain((1..=n).scan(1, |f, i| {
            *f = *f * i % M;
            Some(*f)
        }))
        .collect();

    let inv = (2..=n).fold(vec![1, 1], |mut inv, i| {
        inv.push((M - (M / i) * inv[M % i] % M) % M);
        inv
    });

    let inv_fact: Vec<_> = inv
        .iter()
        .scan(1, |f, i| {
            *f = *f * i % M;
            Some(*f)
        })
        .collect();

    (fact, inv_fact, inv)
}
