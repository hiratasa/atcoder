#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector p(n, 0L), c(n, 0L);
    for (auto&& pp : p) {
        cin >> pp;
        --pp;
    }

    for (auto&& cc : c) {
        cin >> cc;
    }

    int64_t ans = numeric_limits<int64_t>::min();
    for (auto i : irange(0L, n)) {
        int64_t t = 0;
        int64_t pos = i;
        int64_t score = 0;

        vector<int64_t> mx(1L, numeric_limits<int64_t>::min());
        while (true) {
            pos = p[pos];
            score += c[pos];
            mx.emplace_back(max(mx.back(), score));
            ++t;

            if (t == k) {
                break;
            }

            if (pos == i) {
                break;
            }
        }

        ans = max(ans, mx.back());

        if (k - t > 0 && score > 0) {
            ans = max(ans, (k - t) / t * score + mx.back());
            ans = max(ans, k / t * score + max(0L, mx[k % t]));
        }
    }

    cout << ans << endl;
}