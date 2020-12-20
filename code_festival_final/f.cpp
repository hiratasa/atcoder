#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t gcd(int64_t a, int64_t b) {
    if (a == 0) {
        return b;
    }

    return gcd(b % a, a);
}

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> b(n);
    for (auto&& bb : b) {
        cin >> bb;
    }

    vector<bool> f(n);
    for (int64_t i = 0; i < n; ++i) {
        auto g1 = gcd(b[i], b[(i + 1) % n]);
        auto g2 = gcd(b[(i + 1) % n], b[(i + 2) % n]);
        auto g = gcd(b[i] / g1, b[(i + 2) % n] / g2);

        if (g != 1) {
            f[i] = true;
        }
    }

    int64_t i0 = 0;
    for (; i0 < n; ++i0) {
        if (!f[i0] && !f[(i0 + 1) % n]) {
            break;
        }
    }

    if (i0 == n) {
        // redundant?
        int64_t ans = n;
        for (auto i : irange(0L, 3L)) {
            int64_t tmp = 1;
            for (int64_t j = i + 3; j < n - 2 + i; ++j) {
                if (f[j]) {
                    ++tmp;
                    j += 2;
                }
            }
            ans = min(ans, tmp);
        }

        cout << ans << endl;
        return 0;
    }

    i0 += 2;
    int64_t ans = 0;
    for (int64_t i = 0; i < n; ++i) {
        if (f[(i0 + i) % n]) {
            ++ans;
            i += 2;
        }
    }

    cout << ans << endl;
}