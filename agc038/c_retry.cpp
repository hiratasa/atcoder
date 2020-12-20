#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> m;
    for (auto _ : irange(0L, n)) {
        int64_t a;
        cin >> a;
        if (a >= m.size()) {
            m.resize(a + 1);
        }
        ++m[a];
    }

    int64_t a_max = m.size() - 1;

    constexpr auto M = 998244353L;

    vector inv(a_max + 1, 1L);
    for (auto i : irange(2L, a_max + 1)) {
        inv[i] = (M - (M / i) * inv[M % i] % M) % M;
    }

    int64_t ans = 0;
    vector b(a_max + 1, 0L);
    for (auto i : irange(1L, a_max + 1) | reversed) {
        int64_t s = 0;
        for (int64_t j = 1; j * i <= a_max; ++j) {
            s += m[i * j] * (i * j) % M;
            s %= M;
        }

        auto ss = s * s % M;
        for (int64_t j = 2; j * i <= a_max; ++j) {
            ss += M - b[i * j];
            ss %= M;
        }

        b[i] = ss;

        ans += (ss + M - m[i] * i % M * i % M) % M * inv[2] % M * inv[i] % M;
        ans %= M;
    }

    cout << ans << endl;
}