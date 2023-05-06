#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    bitset<200000> bs;
    cin >> bs;

    int64_t k = bs.count();
    int64_t k0 = max(1L, k - 1), k1 = k + 1;
    int64_t r0 = 0, r1 = 0, d0 = 1, d1 = 1;
    vector g(n, 0L);
    for (auto i : irange(0L, n)) {
        r0 += d0 * bs[i];
        r0 %= k0;
        r1 += d1 * bs[i];
        r1 %= k1;

        if (bs[i]) {
            g[i] = (k0 - d0 % k0) % k0;
        } else {
            g[i] = d1 % k1;
        }

        d0 = 2 * d0 % k0;
        d1 = 2 * d1 % k1;
    }

    for (auto i : irange(0L, n) | reversed) {
        bitset<20> m;
        if (bs[i]) {
            if (k == 1) {
                cout << 0 << "\n";
                continue;
            } else {
                m = (r0 + g[i]) % k0;
            }
        } else {
            m = (r1 + g[i]) % k1;
        }

        int64_t ans = 1;
        while (m.any()) {
            m = m.to_ullong() % m.count();
            ++ans;
        }

        cout << ans << "\n";
    }
}