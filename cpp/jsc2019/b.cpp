#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, i)) {
            if (a[i] > a[j]) {
                ans += (k - 1) * k / 2;
                ans %= 1000000007;
            }
        }
        for (auto j : irange(i + 1, n)) {
            if (a[i] > a[j]) {
                ans += (k + 1) * k / 2;
                ans %= 1000000007;
            }
        }
    }

    cout << ans << endl;
}