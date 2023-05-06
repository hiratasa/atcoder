#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    int64_t s = 0;
    for (auto&& aa : a) {
        cin >> aa;
        s += aa;
    }

    constexpr auto M = 998244353L;

    vector<int64_t> dp(s / 2 + 1), dp2(s / 2 + 1);
    dp[0] = 3;
    dp2[0] = 3;
    int64_t ans = 1;
    for (auto aa : a) {
        ans *= 3;
        ans %= M;
        for (auto i : irange(0L, max(0L, s / 2 + 1 - aa)) | reversed) {
            dp[i + aa] += dp[i] * 2;
            dp[i + aa] %= M;
            dp2[i + aa] += dp2[i];
            dp2[i + aa] %= M;
        }
    }

    for (auto i : irange(0L, s / 2 + 1)) {
        ans += M - dp[i];
        ans %= M;
    }

    if (s % 2 == 0) {
        ans += dp2[s / 2];
        ans %= M;
    }

    cout << ans << endl;
}