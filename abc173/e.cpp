#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(0L, 0L);
    bool has_zero = false;
    for (auto _ : irange(0L, n)) {
        int64_t aa;
        cin >> aa;

        if (aa != 0) {
            a.push_back(aa);
        } else {
            has_zero = true;
        }
    }

    n = a.size();
    if (n < k) {
        cout << 0 << endl;
        return 0;
    }

    sort(a.rbegin(), a.rend(),
         [](int64_t x, int64_t y) { return abs(x) < abs(y); });

    int64_t s = 1, prod = 1, prodp = 1, prodn = 1, mp = -1, mn = -1;
    constexpr auto M = 1000000007L;
    for (auto i : irange(0L, k) | reversed) {
        if (a[i] < 0) {
            s *= -1;
            if (mn < 0) {
                mn = -a[i];
            } else {
                prodn *= -a[i];
            }
            prodp *= -a[i];
        } else {
            if (mp < 0) {
                mp = a[i];
            } else {
                prodp *= a[i];
            }
            prodn *= a[i];
        }

        prod *= abs(a[i]);
        prod %= M;
        prodn %= M;
        prodp %= M;
    }

    int64_t ans = 1;
    if (s > 0) {
        ans = prod;
    } else {
        auto itn = find_if(a.begin() + k, a.end(),
                           [](int64_t aa) { return aa < 0; });

        if (mp < 0 || itn == a.end()) {
            prodp = -1;
        } else {
            prodp *= -*itn;
            prodp %= M;
        }

        auto itp = find_if(a.begin() + k, a.end(),
                           [](int64_t aa) { return aa > 0; });

        assert(mn > 0);
        if (itp == a.end()) {
            prodn = -1;
        } else {
            prodn *= *itp;
            prodn %= M;
        }

        if (prodp > 0 && prodn > 0) {
            if (-*itn * mn > *itp * mp) {
                prodn = -1;
            } else {
                prodp = -1;
            }
        }

        if (prodp > 0) {
            ans = prodp;
        } else if (prodn > 0) {
            ans = prodn;
        } else if (has_zero) {
            ans = 0;
        } else {
            assert(itp == a.end() && (k == n || mp < 0));
            for (auto i : irange(0L, k)) {
                ans *= abs(a[n - i - 1]);
                ans %= M;
            }
            ans = (M - ans);
        }
    }

    cout << ans << endl;
}