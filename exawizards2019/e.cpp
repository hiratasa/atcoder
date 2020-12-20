#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t b, w;
    cin >> b >> w;

    constexpr auto M = 1000000007L;
    constexpr auto K = 200001L;

    vector<int64_t> fact(K, 1L), inv_fact(K, 1L), inv(K, 1L),
            inv_pow2(K - 1, 1L);
    for (auto i : irange(2L, K)) {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = (M - inv[M % i] * (M / i) % M) % M;
        inv_fact[i] = inv_fact[i - 1] * inv[i] % M;
        inv_pow2[i - 1] = inv_pow2[i - 2] * inv[2] % M;
    }

    auto combi = [&](int64_t n, int64_t m) {
        if (m > n) {
            return 0L;
        }
        return fact[n] * inv_fact[m] % M * inv_fact[n - m] % M;
    };

    int64_t p_no_black = 0, p_no_white = 0;
    for (auto i : irange(1L, b + w + 1)) {
        p_no_black += combi(i - 2, b - 1) * inv_pow2[i - 1] % M;
        p_no_black %= M;
        p_no_white += combi(i - 2, w - 1) * inv_pow2[i - 1] % M;
        p_no_white %= M;
        auto p_both_left = (1 + (M - p_no_black) + (M - p_no_white)) % M;

        cout << (p_no_white + p_both_left * inv[2]) % M << "\n";
    }
}