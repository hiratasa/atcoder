#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 1000000007L;

int main() {
    int64_t n, c;
    cin >> n >> c;

    vector<int64_t> a(n), b(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i];
    }
    for (auto i : irange(0L, n)) {
        cin >> b[i];
    }

    vector<vector<int64_t>> powsum(401, vector<int64_t>(401));
    for (auto i : irange(1L, 401L)) {
        // i^j
        int64_t i_j = 1;
        for (auto j : irange(0L, 401L)) {
            powsum[j][i] = powsum[j][i - 1] + i_j;
            powsum[j][i] %= M;
            i_j = i_j * i % M;
        }
    }

    vector<vector<int64_t>> dp(n + 1, vector<int64_t>(c + 1));
    dp[0][0] = 1;
    for (auto i : irange(0L, n)) {
        for (auto d : irange(0L, c + 1)) {
            for (auto cc : irange(0L, d + 1)) {
                dp[i + 1][d] +=
                        dp[i][cc] *
                        (powsum[d - cc][b[i]] - powsum[d - cc][a[i] - 1] + M) %
                        M;
                dp[i + 1][d] %= M;
            }
        }
    }

    cout << dp[n][c] << endl;
}