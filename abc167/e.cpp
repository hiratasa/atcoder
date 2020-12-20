#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, k;
    cin >> n >> m >> k;

    constexpr auto M = 998244353L;

    // (m-1)^l
    vector m1p(n + 1, 1L);
    for (auto i : irange(1L, n + 1)) {
        m1p[i] = m1p[i - 1] * (m - 1) % M;
    }

    vector fact(1L << 20, 1L), inv_fact(1L << 20, 1L), inv(1L << 20, 1L);
    for (auto i : irange(2L, 1L << 20)) {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = M - inv[M % i] * (M / i) % M;
        inv_fact[i] = inv_fact[i - 1] * inv[i] % M;
    }

    int64_t ans = 0;
    for (auto l : irange(0L, k + 1)) {
        auto nn = n - l;

        auto t = m * m1p[nn - 1] % M;
        auto tt = fact[nn - 1 + l] * inv_fact[nn - 1] % M * inv_fact[l] % M;

        ans += t * tt % M;
        // cerr << t << "," << tt << endl;
        ans %= M;
    }

    cout << ans << endl;
}