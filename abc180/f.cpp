#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, l;
    cin >> n >> m >> l;

    constexpr auto M = 1000000007L;

    vector inv(n + 1, 1L);
    for (auto i : irange(2L, n + 1)) {
        inv[i] = (M - (M / i) * inv[M % i] % M) % M;
    }

    vector dp(n + 1, vector(m + 1, vector(l + 2, vector(2, 0L))));
    for (auto i : irange(0L, l + 2)) {
        dp[0][0][i][0] = dp[0][0][i][1] = 1;
    }
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m + 1)) {
            for (auto k : irange(1L, l + 1)) {
                {
                    int64_t s = 0;
                    for (int64_t c = 1, f = 1;
                         i - c * k + 1 >= 0 && j - (k - 1) * c >= 0; ++c) {
                        f *= inv[c] * (k == 1 ? 1 : inv[2]) % M;
                        f %= M;
                        auto p = i - c * k + 1;
                        s += dp[p][j - (k - 1) * c][k][0] * f % M;
                        s %= M;
                    }
                    dp[i + 1][j][k][1] = dp[i + 1][j][k][0] + s;
                    dp[i + 1][j][k][1] %= M;
                }
                if (k > 1) {
                    int64_t s = 0;
                    for (int64_t c = 1, f = 1;
                         i - c * k + 1 >= 0 && j - k * c >= 0; ++c) {
                        f *= inv[c] * inv[k] % M * (k == 2 ? 1 : inv[2]) % M;
                        f %= M;
                        auto p = i - c * k + 1;
                        s += dp[p][j - k * c][k][1] * f % M;
                        s %= M;
                    }
                    dp[i + 1][j][k + 1][0] = dp[i + 1][j][k][1] + s;
                    dp[i + 1][j][k + 1][0] %= M;
                } else {
                    dp[i + 1][j][k + 1][0] = dp[i + 1][j][k][1];
                }
            }
        }
    }

    int64_t ans = dp[n][m][l + 1][0] + M - dp[n][m][l][0];
    ans %= M;
    for (auto i : irange(2L, n + 1)) {
        ans *= i;
        ans %= M;
    }

    cout << ans << endl;
}