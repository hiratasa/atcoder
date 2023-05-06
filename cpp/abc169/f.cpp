#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 998244353L;

int main() {
    int64_t n, s;
    cin >> n >> s;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> dp(s + 1);
    dp[0] = 1;
    for (auto i : irange(0L, n)) {
        dp[0] *= 2;
        dp[0] %= M;
    }

    int64_t inv2 = M / 2 + 1;
    for (auto aa : a) {
        for (auto ss : irange(0L, max(0L, s - aa + 1)) | reversed) {
            dp[ss + aa] += dp[ss] * inv2 % M;
            dp[ss + aa] %= M;
        }
    }

    cout << dp[s] << endl;
}