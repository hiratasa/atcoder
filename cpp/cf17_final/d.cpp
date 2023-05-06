#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> hp(n);
    for (auto&& t : hp) {
        cin >> t.first >> t.second;
    }

    sort(hp.begin(), hp.end(), [](const auto& t1, const auto& t2) {
        return t1.first + t1.second < t2.first + t2.second;
    });

    constexpr auto INF = numeric_limits<int64_t>::max();

    vector dp(n + 1, INF);
    dp[0] = 0;
    for (auto i : irange(0L, n)) {
        vector next(n + 1, INF);

        next[0] = 0;
        for (auto m : irange(0L, i + 1)) {
            next[m + 1] = dp[m + 1];
            if (hp[i].first >= dp[m]) {
                next[m + 1] = min(dp[m + 1], dp[m] + hp[i].second);
            }
        }

        dp = std::move(next);
    }

    int64_t ans =
            find_if(dp.rbegin(), dp.rend(), [](int64_t v) { return v < INF; })
                    .base() -
            dp.begin() - 1;
    cout << ans << endl;
}