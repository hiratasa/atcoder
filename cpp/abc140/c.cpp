#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> b(n - 1);
    for (auto i : irange(0L, n - 1)) {
        cin >> b[i];
    }

    int64_t ans = 0;
    ans += b[0];
    for (auto i : irange(1L, n - 1)) {
        ans += min(b[i - 1], b[i]);
    }
    ans += b[n - 2];

    cout << ans << endl;
}