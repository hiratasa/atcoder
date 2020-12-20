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

    vector is_prime(a_max + 1, true);
    vector<int64_t> primes;
    for (auto i = 2L; i <= a_max; ++i) {
        if (is_prime[i]) {
            primes.push_back(i);
        }

        if (i * i <= a_max) {
            for (auto j = 2L; i * j <= a_max; ++j) {
                is_prime[i * j] = false;
            }
        }
    }

    vector b(a_max + 1, 0L);
    for (auto i : irange(1L, a_max + 1) | reversed) {
        int64_t s = 0, s2 = 0;
        for (int64_t j = 1; j * i <= a_max; ++j) {
            s += m[i * j] * (i * j) % M;
            s %= M;
            s2 += m[i * j] * (i * j) % M * (i * j) % M;
            s2 %= M;
        }

        b[i] = (s * s % M + M - s2) % M * inv[2] % M;
    }

    for (auto p : primes) {
        for (auto i = 1; i * p <= a_max; ++i) {
            b[i] += M - b[i * p];
            b[i] %= M;
        }
    }

    int64_t ans = 0;
    for (auto i : irange(1L, a_max + 1)) {
        ans += b[i] * inv[i] % M;
        ans %= M;
    }

    cout << ans << endl;
}