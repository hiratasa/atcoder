// calc sum[x=0 to n-1] floor((a+x*b)/c)
#[allow(dead_code)]
fn floor_sum(n: usize, mut a: usize, mut b: usize, c: usize) -> usize {
    let mut ret = 0;

    // if c < 0 {
    //     return floor_sum(n, -a, -b, -c);
    // }

    ret += a.div_euclid(c) * n;
    a = a.rem_euclid(c);
    // assert!(a >= 0);

    ret += b.div_euclid(c) * n * (n - 1) / 2;
    b = b.rem_euclid(c);
    // assert!(b >= 0);

    if b == 0 {
        return ret;
    }

    // y=(a+x*b)/c に対して、以下のように変数変換して考える
    //  x' = floor((a+n*b)/c) - y
    //  y' = n - x
    //  => y ' = ((a+n*b)%c+x'*c)/b
    // これに対するfloor_sumは元の式に対するfloor_sumと一致する(省略; グラフで格子点の数を考える)
    let last = a + n * b;
    ret += floor_sum(last / c, last % c, c, b);
    ret
}
