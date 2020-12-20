// 最小包含円
// 色々アルゴリズムがあるっぽい。。。（難しい）

// https://tubo28.me/compprog/algorithm/minball/
// コピペってきただけなので要修正
// （シャッフルしているので）確率的にO(N)で動作
template <class iter>
std::pair<P, ld> min_ball(iter left, iter right, int seed = 1333) {
    const int n = right - left;

    assert(n >= 1);
    if (n == 1) {
        return {*left, ld(0)};
    }

    std::mt19937 mt(seed);
    std::shuffle(left, right, mt);
    // std::random_shuffle(left, right); // simple but deprecated

    iter ps = left;
    using circle = std::pair<P, ld>;

    auto make_circle_3 = [](const P &a, const P &b, const P &c) -> circle {
        ld A = std::norm(b - c), B = std::norm(c - a), C = std::norm(a - b),
           S = cross(b - a, c - a);
        P p = (A * (B + C - A) * a + B * (C + A - B) * b +
               C * (A + B - C) * c) /
              (4 * S * S);
        ld r2 = std::norm(p - a);
        return {p, r2};
    };

    auto make_circle_2 = [](const P &a, const P &b) -> circle {
        P c = (a + b) / (ld)2;
        ld r2 = std::norm(a - c);
        return {c, r2};
    };

    auto in_circle = [](const P &a, const circle &c) -> bool {
        return std::norm(a - c.first) <= c.second + eps;
    };

    circle c = make_circle_2(ps[0], ps[1]);

    // MiniDisc
    for (int i = 2; i < n; ++i) {
        if (!in_circle(ps[i], c)) {
            // MiniDiscWithPoint
            c = make_circle_2(ps[0], ps[i]);
            for (int j = 1; j < i; ++j) {
                if (!in_circle(ps[j], c)) {
                    // MiniDiscWith2Points
                    c = make_circle_2(ps[i], ps[j]);
                    for (int k = 0; k < j; ++k) {
                        if (!in_circle(ps[k], c)) {
                            c = make_circle_3(ps[i], ps[j], ps[k]);
                        }
                    }
                }
            }
        }
    }
    return c;
}