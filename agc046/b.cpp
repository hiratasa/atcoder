#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, c, d;
    cin >> a >> b >> c >> d;

    auto e = c - a;
    auto f = d - b;

    constexpr auto M = 998244353L;

    vector dp(f + 1, 0L);
    dp[0] = 1;
    for (auto i : irange(1L, f + 1)) {
        dp[i] = dp[i - 1] * a % M;
    }

    for (auto i : irange(a, c)) {
        for (auto j : irange(0L, f + 1)) {
            dp[j] *= b + j;
            dp[j] %= M;
        }

        vector dp2(f + 1, 0L);
        for (auto j : irange(1L, f + 1)) {
            dp2[j] = dp[j - 1];
        }

        for (auto j : irange(1L, f + 1)) {
            dp2[j] += dp2[j - 1] * (i + 1) % M;
            dp2[j] %= M;
        }

        for (auto j : irange(0L, f + 1)) {
            dp[j] += dp2[j];
            dp[j] %= M;
        }
    }

    cout << dp[f] << endl;
}