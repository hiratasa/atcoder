#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> pl(n);
    for (auto&& t : pl) {
        cin >> t.first >> t.second;
    }

    constexpr auto M = 1000000007L;
    vector<vector<int64_t>> dp(n + 1);
    int64_t pp = -1001;
    dp[0].push_back(1);
    for (auto i : irange(0L, n)) {
        auto p = pl[i].first;
        auto l = pl[i].second;
        int64_t s = 0;
        dp[i + 1].resize(2 * l + 1);
        for (auto x : irange(0L, 2 * l + 1)) {
            auto prev_idx = p - l + x - pp - 1;
            if (prev_idx < 0) {
                continue;
            }
            dp[i + 1][x] = s + (prev_idx < dp[i].size() ? dp[i][prev_idx]
                                                        : dp[i].back());
            dp[i + 1][x] %= M;
            s = dp[i + 1][x];
        }

        pp = p - l;
    }

    // for (auto i : irange(0L, n + 1)) {
    //     for (auto d : dp[i]) {
    //         cerr << d << ",";
    //     }
    //     cerr << endl;
    // }

    cout << dp.back().back() << endl;
}