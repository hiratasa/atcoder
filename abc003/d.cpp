#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t r, c, x, y, d, l;
    cin >> r >> c >> x >> y >> d >> l;

    constexpr auto M = 1000000007L;

    constexpr auto K = 1000L;
    vector<int64_t> fact(K, 1L), inv(K, 1L), inv_fact(K, 1L);
    for (auto i : irange(2L, K)) {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = M - inv[M % i] * (M / i) % M;
        inv_fact[i] = inv_fact[i - 1] * inv[i] % M;
    }

    auto combi = [&](int64_t n, int64_t m) {
        if (n < m) {
            return 0L;
        }
        return fact[n] * inv_fact[m] % M * inv_fact[n - m] % M;
    };

    int64_t t = 0;
    if (x == 1 && y == 1) {
        t = 1;
    } else if (d + l == 1) {
        t = 0;
    } else if (min(x, y) == 1) {
        t = combi(max(x, y) - 2, d + l - 2);
    } else {
        t = combi(x * y, d + l) - 2 * combi((x - 1) * y, d + l) -
            2 * combi(x * (y - 1), d + l) + combi((x - 2) * y, d + l) +
            combi(x * (y - 2), d + l) + 4 * combi((x - 1) * (y - 1), d + l) -
            2 * combi((x - 2) * (y - 1), d + l) -
            2 * combi((x - 1) * (y - 2), d + l) +
            combi((x - 2) * (y - 2), d + l);
    }
    if (t > 0) {
        t = t % M;
    } else {
        t = (M - (-t) % M) % M;
    }

    cerr << t << endl;

    int64_t ans = t * combi(d + l, d) % M * (r - x + 1) % M * (c - y + 1) % M;

    cout << ans << endl;
}