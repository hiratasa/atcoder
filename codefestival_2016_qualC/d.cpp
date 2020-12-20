#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    vector<vector<char>> colors(w, vector<char>(h));
    for (auto i : irange(0L, h)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, w)) {
            colors[j][i] = s[j];
        }
    }

    int64_t ans = 0;
    for (auto i : irange(0L, w - 1)) {
        vector<vector<int64_t>> dp(h + 1, vector<int64_t>(h + 1)),
                dp2(h + 1, vector<int64_t>(h + 1));

        for (auto j : irange(1L, h + 1)) {
            for (auto k : irange(1L, h + 1)) {
                dp[j][k] = dp[j - 1][k - 1] +
                           (colors[i][j - 1] == colors[i + 1][k - 1]);
                dp2[j][k] = dp[j][k] + min(dp2[j - 1][k], dp2[j][k - 1]);
            }
        }

        ans += dp2[h][h];
    }

    cout << ans << endl;
}