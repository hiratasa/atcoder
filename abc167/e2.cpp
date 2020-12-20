#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 998244353L;

int64_t solve_fixed(int64_t n, int64_t m, int64_t k) {
    static unordered_map<int64_t, unordered_map<int64_t, int64_t>> memo;

    // m does not change
    if (memo[n].count(k) > 0) {
        return memo[n][k];
    }

    if (n == 1) {
        if (k == 0) {
            return 1;
        } else {
            return 0;
        }
    }

    if (k < 0) {
        return 0;
    }

    if (n % 2 == 1) {
        memo[n][k] = (solve_fixed(n - 1, m, k - 1) % M +
                      (m - 1) % M * solve_fixed(n - 1, m, k) % M) %
                     M;
        // cerr << n << "," << k << ":" << memo[n][k] << endl;
        return memo[n][k];
    }

    int64_t ret = 0;
    for (auto i : irange(0L, k + 1)) {
        ret += solve_fixed(n / 2, m, i) * solve_fixed(n / 2, m, k - i) % M *
               (m - 1) % M;
        ret += solve_fixed(n / 2, m, i) * solve_fixed(n / 2, m, k - 1 - i) % M;
        ret %= M;
    }

    // cerr << n << "," << k << ":" << ret << endl;
    return memo[n][k] = ret;
}

int64_t solve(int64_t n, int64_t m, int64_t k) {
    return m * solve_fixed(n, m, k) % M;
}

int main() {
    int64_t n, m, k;
    cin >> n >> m >> k;

    int64_t ans = 0;
    for (auto i : irange(0L, k + 1)) {
        ans += solve(n, m, i);
        ans %= M;
    }

    cout << ans << endl;
}