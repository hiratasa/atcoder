#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t f(const vector<int64_t>& a, int64_t m) {
    int64_t n = a.size();
    vector<int64_t> b(n);
    int64_t l = 0;
    for (auto i : irange(0L, n)) {
        b[i] = a[i] % m;
        l += b[i];
    }
    l /= m;

    sort(b.rbegin(), b.rend());

    int64_t c = 0;
    for (auto i : irange(0L, l)) {
        c += m - b[i];
    }
    for (auto i : irange(l, n)) {
        c += b[i];
    }

    return c / 2;
}

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> a(n);
    int64_t s = 0;
    for (auto&& aa : a) {
        cin >> aa;
        s += aa;
    }

    int64_t ans = -1;
    for (int64_t m = 1; m * m <= s; ++m) {
        if (s % m > 0) {
            continue;
        }

        auto c = f(a, m);
        if (c <= k) {
            ans = max(ans, m);
        }

        auto m2 = s / m;
        auto c2 = f(a, m2);
        if (c2 <= k) {
            ans = max(ans, m2);
        }
    }

    cout << ans << endl;
}