#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    constexpr auto M = 1000000007L;

    if (n == 1) {
        cout << 1 << endl;
        return 0;

    } else if (n == 2) {
        cout << 4 << endl;
        return 0;
    } else if (n == 3) {
        cout << 15 << endl;
        return 0;
    }

    // vector dp(n + 1, 0L), dp2(n + 1, 0L), dp3(n + 1, 0L);
    // dp[0] = dp2[0] = dp3[0] = 1;
    // dp[1] = 0;
    // dp2[1] = 1;
    // dp3[1] = 2;
    // dp[2] = 0;
    // dp2[2] = 1;
    // dp3[2] = 3;
    // int64_t ans = (3 * n * (n - 1) + 1) % M;
    // for (auto i : irange(3L, n)) {
    //     dp[i] = dp3[i - 3];
    //     dp2[i] = (dp2[i - 1] + dp[i]) % M;
    //     dp3[i] = (dp3[i - 1] + dp2[i]) % M;

    //     if (i <= n - 2) {
    //         ans += dp2[i] * (n - 1) % M * n % M;
    //     } else if (i == n - 1) {
    //         ans += dp2[i] * (n - 1) % M;
    //     }
    //     ans %= M;
    // }

    vector dp(n + 1, 0L), dp2(n + 1, 0L);
    dp[0] = dp2[0] = 1;
    dp[1] = 1;
    dp2[1] = 2;
    for (auto i : irange(2L, n)) {
        dp[i] = (dp2[i - 1] + M - dp[i - 2]) % M;
        dp2[i] = (dp2[i - 1] + dp[i]) % M;
    }
    int64_t ans =
            1 + dp2[n - 2] * (n - 1) % M * n % M + dp[n - 1] * (n - 1) % M;
    ans %= M;

    cout << ans << endl;
}