#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t h, w;
    cin >> h >> w;

    vector<vector<int64_t>> a(h + 1, vector<int64_t>(w + 1)),
            b(h + 1, vector<int64_t>(w + 1));
    for (auto i : irange(1L, h + 1L)) {
        for (auto j : irange(1L, w + 1L)) {
            cin >> a[i][j];
        }
    }
    for (auto i : irange(1L, h + 1L)) {
        for (auto j : irange(1L, w + 1L)) {
            cin >> b[i][j];
        }
    }

    constexpr auto kOffset = 160 * 80;
    vector<vector<bitset<2 * 160 * 80 + 1>>> dp(
            h + 1L, vector<bitset<2 * 160 * 80 + 1>>(w + 1L));
    dp[0][1].set(kOffset);
    for (auto i : irange(1L, h + 1L)) {
        for (auto j : irange(1L, w + 1L)) {
            auto diff = a[i][j] - b[i][j];
            if (diff < 0) {
                diff *= -1;
            }

            dp[i][j] = (dp[i - 1][j] << diff);
            dp[i][j] |= (dp[i - 1][j] >> diff);
            dp[i][j] |= (dp[i][j - 1] << diff);
            dp[i][j] |= (dp[i][j - 1] >> diff);
        }
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto i = 0L; i <= 2 * 160 * 80; ++i) {
        if (dp[h][w][i]) {
            ans = min(ans, abs(i - kOffset));
        }
    }

    cout << ans << endl;
}