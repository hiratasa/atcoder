#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    vector a(1, 0L);
    int64_t t = 0;
    for (auto c : s) {
        if (c == '0') {
            a.push_back(t);
        } else {
            ++t;
        }
    }
    a.push_back(t);

    int64_t k;
    cin >> k;
    k = min(k, a.back());

    int64_t n = a.size();

    constexpr auto M = 998244353L;

    vector dp(k + 1, vector(a.back() + 1, 0L));
    dp[0][0] = 1;
    for (auto i : irange(0L, n - 1)) {
        vector next(k + 1, vector(a.back() + 1, 0L));

        for (auto j : irange(0L, k + 1)) {
            int64_t ss = 0;
            for (auto c : irange(a[i + 1], a.back() + 1)) {
                next[j][c] += ss + dp[j][a[i] + (c - a[i + 1])];
                ss += dp[j][a[i] + (c - a[i + 1])];
                next[j][c] %= M;
            }

            for (auto c : irange(a[i + 1], a.back() + 1)) {
                for (auto b : irange(a[i] + (c - a[i + 1]) + 1, c + 1)) {
                    auto w = j + b - (a[i] + (c - a[i + 1]));
                    if (w > k) {
                        break;
                    }
                    next[w][c] += dp[j][b];
                    next[w][c] %= M;
                }
            }
        }

        dp = std::move(next);
    }

    int64_t ans = 0;
    for (auto i : irange(0L, k + 1)) {
        ans += dp[i][a.back()];
        ans %= M;
    }

    cout << ans << endl;
}