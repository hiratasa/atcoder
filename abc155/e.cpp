#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    transform(s.begin(), s.end(), s.begin(), [](char c) { return c - '0'; });

    int64_t n = s.size();
    vector<vector<int64_t>> dp(2, vector<int64_t>(n + 1));
    dp[0][0] = 0;
    dp[1][0] = 1;
    for (auto i : irange(0L, n)) {
        dp[0][i + 1] = min(dp[0][i] + s[i], dp[1][i] + (10 - s[i]));
        dp[1][i + 1] = min({dp[0][i] + s[i] + 1, dp[1][i] + (10 - s[i] - 1)});
    }

    cout << dp[0][n] << endl;
}