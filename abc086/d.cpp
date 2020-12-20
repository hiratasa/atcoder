#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector<vector<int64_t>> f(k + 1, vector<int64_t>(k + 1));
    for (auto _ : irange(0L, n)) {
        int64_t x, y;
        char c;
        cin >> x >> y >> c;

        assert(c == 'B' || c == 'W');

        if (c == 'W') {
            x += k;
        }

        if ((x / k + y / k) % 2 == 0) {
            ++f[x % k + 1][y % k + 1];
        } else {
            --f[x % k + 1][y % k + 1];
        }
    }

    for (auto x : irange(0L, k + 1)) {
        for (auto y : irange(0L, k)) {
            f[x][y + 1] += f[x][y];
        }
    }

    for (auto x : irange(0L, k)) {
        for (auto y : irange(0L, k + 1)) {
            f[x + 1][y] += f[x][y];
        }
    }

    auto nb = (n + f[k][k]) / 2;
    auto nw = (n - f[k][k]) / 2;

    int64_t ans = 0;
    for (auto x : irange(0L, k)) {
        for (auto y : irange(0L, k)) {
            int64_t t = f[k][k] - f[x][k] - f[k][y] + 2 * f[x][y];
            ans = max(ans, nw + t);
            ans = max(ans, nb - t);
        }
    }

    cout << ans << endl;
}