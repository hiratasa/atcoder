#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    constexpr auto M = 1000000007;
    constexpr auto K = 13L;
    int64_t base = 1;
    array<int64_t, K> dp{};
    dp[0] = 1;
    for (auto it = s.rbegin(); it != s.rend(); ++it) {
        auto c = *it;

        array<int64_t, K> next{};
        if (c != '?') {
            auto d = (base * (c - '0')) % K;
            for (auto i : irange(0L, K)) {
                next[(i + d) % K] = dp[i];
            }
        } else {
            for (auto i : irange(0L, 10L)) {
                auto d = (base * i) % K;
                for (auto j : irange(0L, K)) {
                    next[(j + d) % K] += dp[j];
                    next[(j + d) % K] %= M;
                }
            }
        }

        dp = next;
        base *= 10;
        base %= K;
    }

    cout << dp[5] << endl;
}