#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> v(n);
    for (auto&& vv : v) {
        cin >> vv;
    }

    sort(v.begin(), v.end());

    double ans = v[0];
    for (auto i : irange(1L, n)) {
        ans = (ans + v[i]) / 2.0;
    }

    cout << ans << endl;
}