#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    vector<vector<bool>> s(h + 1, vector<bool>(w + 1));
    for (auto i : irange(0L, h)) {
        string ss;
        cin >> ss;
        for (auto j : irange(0L, w)) {
            s[i + 1][j + 1] = (ss[j] == '#');
        }
    }

    int64_t dxs[] = {0L, -1L};
    int64_t dys[] = {-1L, 0L};

    vector<vector<int64_t>> dp(
            h + 1, vector<int64_t>(w + 1, numeric_limits<int64_t>::max() / 2));
    dp[0][1] = 0;
    dp[1][0] = 0;
    for (auto i : irange(1L, h + 1)) {
        for (auto j : irange(1L, w + 1)) {
            for (auto k : {0, 1}) {
                auto dx = dxs[k];
                auto dy = dys[k];

                if (!s[i][j] || (s[i][j] == s[i + dy][j + dx])) {
                    dp[i][j] = min(dp[i][j], dp[i + dy][j + dx]);
                } else {
                    dp[i][j] = min(dp[i][j], dp[i + dy][j + dx] + 1);
                }
            }
        }
    }

    cout << dp[h][w] << endl;
}