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

    vector<vector<vector<bool>>> dp(
            h + 1L, vector<vector<bool>>(w + 1L, vector<bool>(160 * 80 + 1)));
    dp[0][1][0] = true;
    for (auto i : irange(1L, h + 1L)) {
        for (auto j : irange(1L, w + 1L)) {
            auto diff = a[i][j] - b[i][j];

            for (auto k : irange(0L, 160 * 80 + 1L)) {
                if (!dp[i - 1][j][k]) {
                    continue;
                }

                for (auto d : {k + diff, k - diff}) {
                    if (d < 0) {
                        d *= -1;
                    }
                    if (d < dp[i][j].size()) {
                        dp[i][j][d] = true;
                    }
                }
            }

            for (auto k : irange(0L, 160 * 80 + 1L)) {
                if (!dp[i][j - 1][k]) {
                    continue;
                }

                for (auto d : {k + diff, k - diff}) {
                    if (d < 0) {
                        d *= -1;
                    }
                    if (d < dp[i][j].size()) {
                        dp[i][j][d] = true;
                    }
                }
            }
        }
    }

    auto ans = find(dp[h][w].begin(), dp[h][w].end(), true) - dp[h][w].begin();

    cout << ans << endl;
}
