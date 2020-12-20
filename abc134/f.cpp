#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

constexpr auto M = 1000000007;

int64_t fact(int64_t k) {
    if (k <= 1) {
        return 1;
    }

    return k * fact(k - 1) % M;
}

main() {
    int64_t n, k;
    cin >> n >> k;

    auto f = fact(n - 1);
    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        for (auto p : irange(0L, n)) {
            ans += abs(i - p) * f;
            ans %= M;
        }
    }

    cout << ans << endl;
}