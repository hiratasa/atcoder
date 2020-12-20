#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n), b(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    for (auto&& bb : b) {
        cin >> bb;
    }

    vector<int64_t> diff(n);
    int64_t sum_diff = 0;
    for (auto i : irange(0L, n)) {
        diff[i] = a[i] - b[i];
        sum_diff += diff[i];
    }

    if (sum_diff < 0) {
        cout << "-1" << endl;
        return 0;
    }

    sort(diff.begin(), diff.end());
    int64_t neg = 0;
    int64_t ans = 0;
    for (auto d : diff) {
        if (d >= 0) {
            break;
        }
        neg += -d;
        ++ans;
    }

    for (auto i : irange(n-1, -1L, -1L)) {
        if (neg <= 0) {
            break;
        }
        auto d = diff[i];
        neg -= d;
        ++ans;
    }

    cout << ans << endl;
}