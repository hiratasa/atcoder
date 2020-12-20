#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t solve(const vector<pair<int64_t, int64_t>>& vw,
              const vector<vector<int64_t>>& dp, int64_t v, int64_t l) {
    int64_t m = dp.size();

    if (l < 0) {
        return numeric_limits<int64_t>::min();
    }

    if (v < m) {
        return dp[v][l];
    }

    return max(solve(vw, dp, (v - 1) / 2, l),
               solve(vw, dp, (v - 1) / 2, l - vw[v].second) + vw[v].first);
}

int main() {
    int64_t n;
    cin >> n;

    vector vw(n, make_pair(0L, 0L));
    for (auto&& t : vw) {
        cin >> t.first >> t.second;
    }

    auto m = min(512L, n);

    vector dp(m, vector(100001L, 0L));
    for (auto j : irange(vw[0].second, 100001L)) {
        dp[0][j] = vw[0].first;
    }
    for (auto i : irange(1L, m)) {
        for (auto j : irange(0L, vw[i].second)) {
            dp[i][j] = dp[(i - 1) / 2][j];
        }
        for (auto j : irange(vw[i].second, 100001L)) {
            dp[i][j] = max(dp[(i - 1) / 2][j],
                           dp[(i - 1) / 2][j - vw[i].second] + vw[i].first);
        }
    }

    int64_t q;
    cin >> q;
    for (auto _ : irange(0L, q)) {
        int64_t v, l;
        cin >> v >> l;
        --v;

        cout << solve(vw, dp, v, l) << "\n";
    }
}