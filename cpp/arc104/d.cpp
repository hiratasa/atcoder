#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k, m;
    cin >> n >> k >> m;

    vector dp(n, vector(n * (n - 1) / 2 * k + 1, 0L));
    dp[0][0] = 1;
    for (auto i : irange(1L, n)) {
        dp[i] = dp[i - 1];

        for (auto j : irange(i, n * (n - 1) / 2 * k + 1)) {
            dp[i][j] += dp[i][j - i];
            dp[i][j] %= m;
        }

        for (auto j : irange(i * (k + 1), n * (n - 1) / 2 * k + 1) | reversed) {
            dp[i][j] += m - dp[i][j - i * (k + 1)];
            dp[i][j] %= m;
        }
    }

    for (auto x : irange(1L, n + 1)) {
        const auto& dpl = dp[x - 1];
        const auto& dpr = dp[n - x];

        int64_t ans = 0;
        for (auto i : irange(0L, n * (n - 1) / 2 * k + 1)) {
            ans += dpl[i] * dpr[i] % m;
            ans %= m;
        }
        ans = ans * (k + 1) % m;
        ans += m - 1;
        ans %= m;

        cout << ans << "\n";
    }
}