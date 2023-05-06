#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    constexpr auto M = 1000000007L;

    vector<int64_t> dp(4);
    dp[0] = 1;
    for (auto c : s) {
        if (c == '?') {
            dp[3] = 3 * dp[3] + dp[2];
            dp[3] %= M;
            dp[2] = 3 * dp[2] + dp[1];
            dp[2] %= M;
            dp[1] = 3 * dp[1] + dp[0];
            dp[1] %= M;
            dp[0] = 3 * dp[0];
            dp[0] %= M;
        }
        if (c == 'C') {
            dp[3] += dp[2];
            dp[3] %= M;
        }
        if (c == 'B') {
            dp[2] += dp[1];
            dp[2] %= M;
        }
        if (c == 'A') {
            dp[1] += dp[0];
            dp[1] %= M;
        }
        cerr << dp[0] << "," << dp[1] << "," << dp[2] << "," << dp[3] << endl;
    }

    cout << dp[3] << endl;
}