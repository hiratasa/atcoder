#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 1000000007L;

int64_t mpow(int64_t x, int64_t y) {
    if (y == 0) {
        return 1;
    }

    if (y == 1) {
        return x;
    }

    auto t = mpow(x, y / 2);

    return t * t % M * mpow(x, y % 2) % M;
}

int main() {
    int64_t k;
    cin >> k;

    string s;
    cin >> s;

    int64_t n = s.size();

    constexpr auto B = 2000001L;

    vector fact(B, 1L), ifact(B, 1L), inv(B, 1L);
    for (auto i : irange(2L, B)) {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = (M - inv[M % i] * (M / i) % M) % M;
        ifact[i] = ifact[i - 1] * inv[i] % M;
    }

    auto combi = [&](int64_t x, int64_t y) {
        return fact[x] * ifact[y] % M * ifact[x - y] % M;
    };

    int64_t ans = 0;
    for (auto i : irange(0L, k + 1)) {
        auto r = k - i;

        ans += mpow(26, i) * mpow(25, r) % M * combi(n + r - 1, r) % M;
        ans %= M;
    }

    cout << ans << endl;
}