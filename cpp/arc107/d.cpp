#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    constexpr auto M = 998244353L;

    vector dp(n + 1, vector(n + 1, 0L));
    dp[0][0] = 1;
    for (auto m : irange(1L, n + 1)) {
        for (auto kk : irange(1L, n + 1) | reversed) {
            dp[m][kk] = dp[m - 1][kk - 1];
            if (2 * kk <= m) {
                dp[m][kk] += dp[m][2 * kk];
            }
            dp[m][kk] %= M;
        }
    }

    cout << dp[n][k] << endl;
}