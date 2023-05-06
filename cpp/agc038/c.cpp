#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 998244353;

int64_t pow_mod(int64_t x, int64_t y) {
    if (y == 0) {
        return 1;
    }

    if (y == 1) {
        return x;
    }

    auto p = pow_mod(x, y / 2);

    return ((p * p % M) * pow_mod(x, y % 2)) % M;
}

int64_t inv(int64_t x) { return pow_mod(x, M - 2); }

int64_t calc(const vector<int64_t>& ss, const vector<int64_t>& dp, int64_t g) {
    int64_t ret = 0;

    ret += ss[g];

    for (auto gg = 2 * g; gg < dp.size(); gg += g) {
        ret += (-dp[gg] + M) * gg;
        ret %= M;
    }

    return ret * inv(g) % M;
}

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> p(1000001L);
    int64_t mx = -1;
    for (auto _ : irange(0L, n)) {
        int64_t a;
        cin >> a;

        ++p[a];
        mx = max(mx, a);
    }

    int64_t inv2 = inv(2);
    vector<int64_t> ss(mx + 1);
    for (auto i : irange(1L, mx + 1)) {
        int64_t s = 0;
        int64_t s2 = 0;
        for (int64_t j = 1; i * j <= mx; ++j) {
            s += (i * j) * p[i * j];
            s %= M;
            s2 += (i * j) * (i * j) * p[i * j];
            s2 %= M;
        }

        ss[i] = (s * s - s2 + M) % M * inv2;
        ss[i] %= M;
    }

    int64_t ans = 0;
    vector<int64_t> dp(mx + 1);
    for (auto g : irange(1L, mx + 1) | reversed) {
        dp[g] = calc(ss, dp, g);
        ans += dp[g];
        ans %= M;
    }

    cout << ans << endl;
}