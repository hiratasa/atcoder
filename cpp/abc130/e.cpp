#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> s(n), t(m);
    for (auto&& ss : s) {
        cin >> ss;
    }

    for (auto&& tt : t) {
        cin >> tt;
    }

    constexpr auto M = 1000000007;

    vector<vector<int64_t>> dp(n + 1, vector<int64_t>(m + 1));
    vector<vector<int64_t>> dp2(n + 1, vector<int64_t>(m + 1));
    vector<vector<int64_t>> dp3(n + 1, vector<int64_t>(m + 1));
    for (auto i : irange(0L, n + 1)) {
        dp[i][0] = 1;
        dp2[i][0] = (i == 0 ? 1 : 0);
        dp3[i][0] = 1;
    }
    for (auto j : irange(0L, m + 1)) {
        dp[0][j] = 1;
        dp2[0][j] = 1;
        dp3[0][j] = (j == 0 ? 1 : 0);
    }
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m)) {
            auto f = (s[i] == t[j] ? dp[i][j] : 0);
            dp[i + 1][j + 1] = (dp[i][j] + dp2[i + 1][j] + dp3[i][j + 1] + f) % M;
            dp2[i + 1][j + 1] = (dp2[i + 1][j] + f) % M;
            dp3[i + 1][j + 1] = (dp3[i][j + 1] + f) % M;
        }
    }

    #if 0
    for (auto i : irange(0L, n + 1)) {
        for (auto j : irange(0L, m + 1)) {
            cout << dp[i][j] << " ";
        }
        cout << endl;
    }

    for (auto i : irange(0L, n + 1)) {
        for (auto j : irange(0L, m + 1)) {
            cout << dp2[i][j] << " ";
        }
        cout << endl;
    }

    for (auto i : irange(0L, n + 1)) {
        for (auto j : irange(0L, m + 1)) {
            cout << dp3[i][j] << " ";
        }
        cout << endl;
    }
    #endif

    cout << dp[n][m] << endl;
}