#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n), b(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i];
    }
    for (auto i : irange(0L, n)) {
        cin >> b[i];
    }

    int64_t ans = 0;
    for (auto i : irange(0L, 30L)) {
        vector<int64_t> c(n), d(n);
        for (auto j : irange(0L, n)) {
            c[j] = (a[j] & ((1L << i) - 1));
            d[j] = (b[j] & ((1L << i) - 1));
        }
        sort(c.begin(), c.end());
        sort(d.begin(), d.end());
        int64_t carry_over = 0;
        for (auto cc : c) {
            carry_over +=
                    d.end() - lower_bound(d.begin(), d.end(), (1L << i) - cc);
        }

        int64_t na = 0, nb = 0;
        for (auto j : irange(0L, n)) {
            if ((a[j] & (1 << i))) {
                ++na;
            }
            if ((b[j] & (1 << i))) {
                ++nb;
            }
        }

        auto n1 = na * (n - nb) + (n - na) * nb + carry_over;
        if (n1 % 2 > 0) {
            ans |= (1L << i);
        }
    }

    cout << ans << endl;
}