#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 1000000007L;

const vector<int64_t>& factors(int64_t m) {
    static unordered_map<int64_t, vector<int64_t>> memo;

    if (memo.count(m) > 0) {
        return memo[m];
    }

    for (int64_t i = 2; i * i <= m; ++i) {
        if (m % i == 0) {
            memo[m].push_back(i);
            if (i != m / i) {
                memo[m].push_back(m / i);
            }
        }
    }

    memo[m].push_back(m);

    return memo[m];
}

template <typename Combi>
int64_t solve(const Combi& combi, int64_t n, int64_t m, int64_t d) {
    static unordered_map<pair<int64_t, int64_t>, int64_t,
                         boost::hash<std::pair<int64_t, int64_t>>>
            memo;

    if (d > n) {
        return 0L;
    }

    if (memo.count({m, d}) > 0) {
        return memo[{m, d}];
    }

    if (m == 1) {
        // C(n, d)
        return memo[{m, d}] = combi(n, d);
    }

    int64_t ret = 0;
    for (auto f : factors(m)) {
        ret += solve(combi, n, m / f, d + 1);
        ret %= M;
    }

    return memo[{m, d}] = ret;
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> fact(100001L, 1L), inv_fact(100001L, 1L), inv(100001L, 1L);
    for (auto i : irange(2L, 100001L)) {
        fact[i] = fact[i - 1] * i % M;
        inv[i] = (M - inv[M % i] * (M / i) % M) % M;
        inv_fact[i] = inv_fact[i - 1] * inv[i] % M;
    }

    auto combi = [&](int64_t x, int64_t y) {
        if (y > x) {
            return 0L;
        }

        return fact[x] * inv_fact[y] % M * inv_fact[x - y] % M;
    };

    cout << solve(combi, n, m, 0L) << endl;
}