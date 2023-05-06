#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, x;
    cin >> n >> m >> x;

    vector c(n, 0L);
    vector a(n, vector(m, 0L));
    for (auto i : irange(0L, n)) {
        cin >> c[i];
        for (auto j : irange(0L, m)) {
            cin >> a[i][j];
        }
    }

    int64_t ans = 1L << 30;
    for (auto u : irange(0uL, 1uL << n)) {
        bitset<12> bs(u);

        vector b(m, 0L);
        int64_t cc = 0;
        for (auto i : irange(0L, n)) {
            if (!bs[i]) {
                continue;
            }

            cc += c[i];
            for (auto j : irange(0L, m)) {
                b[j] += a[i][j];
            }
        }

        if (all_of(b.begin(), b.end(), [&](int64_t bb) { return bb >= x; })) {
            ans = min(ans, cc);
        }
    }

    if (ans == 1L << 30) {
        cout << -1 << endl;
    } else {
        cout << ans << endl;
    }
}