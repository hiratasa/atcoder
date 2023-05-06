#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t r, c, k;
    cin >> r >> c >> k;

    vector items(r + 1, vector(c + 1, 0L));
    for (auto i : irange(0L, k)) {
        int64_t rr, cc, v;
        cin >> rr >> cc >> v;
        items[rr][cc] = v;
    }

    vector dp(r + 1, vector(c + 1, array<int64_t, 4>{}));
    for (auto i : irange(1L, r + 1)) {
        for (auto j : irange(1L, c + 1)) {
            for (auto t : irange(0L, 4L)) {
                dp[i][j][t] =
                        max({dp[i][j - 1][t], dp[i - 1][j][0], dp[i - 1][j][1],
                             dp[i - 1][j][2], dp[i - 1][j][3]});
            }
            for (auto t : irange(1L, 4L) | reversed) {
                dp[i][j][t] = max(dp[i][j][t], dp[i][j][t - 1] + items[i][j]);
            }
        }
    }

    cout << max({dp[r][c][0], dp[r][c][1], dp[r][c][2], dp[r][c][3]}) << endl;
}