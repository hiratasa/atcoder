#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    constexpr auto P = 200003L;
    vector a(P, 0L);
    for (auto _ : irange(0L, n)) {
        int64_t aa;
        cin >> aa;
        ++a[aa];
    }

    a[0] = 0;

    vector s(P, 0L), m(P + 1, 0L);
    for (auto i : irange(0L, P)) {
        s[i + 1] = s[i] + i * a[i];
        m[i + 1] = m[i] + a[i];
    }

    int64_t ans = 0;
    for (auto i : irange(1L, P)) {
        if (a[i] == 0) {
            continue;
        }

        for (int64_t j = 0;; ++j) {
            int64_t b = (j * P + i - 1) / i;
            int64_t e = ((j + 1) * P + i - 1) / i;

            if (b > P) {
                break;
            }

            e = min(P, e);

            ans += ((s[e] - s[b]) * i - j * P * (m[e] - m[b])) * a[i];
        }

        ans -= i * i % P * a[i];
    }

    cout << ans / 2 << endl;
}