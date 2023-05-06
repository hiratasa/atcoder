#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> h(n);
    for (auto i : irange(0L, n)) {
        cin >> h[i];
    }

    vector<unordered_map<int64_t, int64_t>> dp(k + 1);
    dp[0][0] = 0;
    for (auto i : irange(0L, n)) {
        decltype(dp) next(k + 1);

        for (auto j : irange(0L, k + 1)) {
            for (const auto& row : dp[j]) {
                int64_t val;
                if (row.first < h[i]) {
                    val = row.second + (h[i] - row.first);
                } else {
                    val = row.second;
                }

                if (next[j].count(h[i]) > 0) {
                    next[j][h[i]] = min(next[j][h[i]], val);
                } else {
                    next[j][h[i]] = val;
                }
            }
        }

        for (auto j : irange(1L, k + 1)) {
            for (const auto& row : dp[j - 1]) {
                if (next[j].count(row.first) > 0) {
                    next[j][row.first] = min(next[j][row.first], row.second);
                } else {
                    next[j][row.first] = row.second;
                }
            }
        }

        dp = std::move(next);
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto i : irange(0L, k + 1)) {
        for (auto kv : dp[i]) {
            ans = min(kv.second, ans);
        }
    }

    cout << ans << endl;
}