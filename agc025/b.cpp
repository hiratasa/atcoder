#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    constexpr auto M = 998244353L;

    int64_t n, a, b, k;
    cin >> n >> a >> b >> k;

    vector<int64_t> fact(n + 1, 1L), ifact(n + 1, 1L), inv(n + 1, 1L);
    for (auto i : irange(2L, n + 1)) {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = (M - inv[M % i] * (M / i) % M) % M;
        ifact[i] = ifact[i - 1] * inv[i] % M;
    }

    auto combi = [&](int64_t x, int64_t y) {
        return fact[x] * ifact[y] % M * ifact[x - y] % M;
    };

    int64_t ans = 0;
    for (auto i : irange(0L, n + 1)) {
        if (i * a > k) {
            break;
        }

        int64_t j = (k - i * a) / b;
        if (i * a + j * b != k || j > n) {
            continue;
        }

        ans += combi(n, i) * combi(n, j) % M;
        ans %= M;
    }

    cout << ans << endl;
}