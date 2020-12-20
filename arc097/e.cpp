#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector colors(2 * n, false);
    vector a(2 * n, 0L), b(2 * n, 0L);
    for (auto i : irange(0L, 2 * n)) {
        char c;
        cin >> c >> a[i];
        --a[i];
        colors[i] = (c == 'B');
        b[colors[i] * n + a[i]] = i;
    }

    vector s(2L, vector(2 * n + 1, vector(n + 1, 0L)));
    for (auto i : irange(0L, 2 * n) | reversed) {
        s[0][i] = s[0][i + 1];
        s[1][i] = s[1][i + 1];

        for (auto j : irange(a[i] + 1, n + 1)) {
            ++s[colors[i]][i][j];
        }
    }

    vector dp(n + 1, vector(n + 1, numeric_limits<int32_t>::max()));
    dp[0][0] = 0;
    for (auto j : irange(1L, n + 1)) {
        auto idx = b[n + j - 1];
        dp[0][j] = dp[0][j - 1] + s[1][idx][j - 1];
    }
    for (auto i : irange(1L, n + 1)) {
        auto idx0 = b[i - 1];
        dp[i][0] = dp[i - 1][0] + s[0][idx0][i - 1];

        for (auto j : irange(1L, n + 1)) {
            auto idx1 = b[i - 1];
            auto idx2 = b[n + j - 1];
            dp[i][j] = min(dp[i - 1][j] + s[0][idx1][i - 1] + s[1][idx1][j],
                           dp[i][j - 1] + s[0][idx2][i] + s[1][idx2][j - 1]);
        }
    }

    cout << dp[n][n] << endl;
}