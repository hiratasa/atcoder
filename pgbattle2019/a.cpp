#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(2 * n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    sort(a.begin(), a.end());

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto i : irange(0L, n)) {
        ans = min(ans, a[i + n] - a[i]);
    }

    cout << ans << endl;
}