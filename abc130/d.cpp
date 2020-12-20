#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> a(n);
    int64_t j = 0;
    int64_t ss = 0;
    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        cin >> a[i];

        ss += a[i];
        while (ss - a[j] >= k) {
            ss -= a[j];
            ++j;
        }

        if (ss >= k) {
            ans += j + 1;
        }
    }

    cout << ans << endl;
}