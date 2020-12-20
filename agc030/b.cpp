#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t l, n;
    cin >> l >> n;
    vector x(n, 0L);
    for (auto&& xx : x) {
        cin >> xx;
    }
    vector s(1, 0L);
    for (auto xx : x) {
        s.push_back(s.back() + xx);
    }
    int64_t ans = 0;
    for (auto i : irange(0L, n + 1)) {
        if (i < n - i) {
            if (i > 0) {
                int64_t a = s[i] + s[i - 1];
                int64_t b = 2 * (i * l - (s[2 * i] - s[i]));
                ans = max(ans, a + b);
            }

            {
                int64_t a = 2 * s[i];
                int64_t b = (2 * i + 1) * l - (s[2 * i + 1] - s[i + 1]) -
                            (s[2 * i + 1] - s[i]);
                ans = max(ans, a + b);
            }
        } else if (i == n - i) {
            {
                int64_t a = s[i] + s[i - 1];
                int64_t b = 2 * (i * l - (s[2 * i] - s[i]));
                ans = max(ans, a + b);
            }

            {
                int64_t a = 2 * s[i];
                int64_t b = (2 * i - 1) * l - (s[2 * i] - s[i + 1]) -
                            (s[2 * i] - s[i]);
                ans = max(ans, a + b);
            }
        } else {
            assert(i > n - i);
            {
                int64_t a = (s[i] - s[i - (n - i + 1)]) +
                            (s[i - 1] - s[i - (n - i + 1)]);
                int64_t b = 2 * ((n - i) * l - (s[n] - s[i]));
                ans = max(ans, a + b);
            }

            if (i < n) {
                int64_t a = 2 * (s[i] - s[i - (n - i)]);
                int64_t b = (2 * (n - i) - 1) * l - (s[n] - s[i + 1]) -
                            (s[n] - s[i]);
                ans = max(ans, a + b);
            }
        }
    }
    cout << ans << endl;
}
