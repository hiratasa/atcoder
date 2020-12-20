#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> l(n + 1), r(n + 1);
    for (auto i : irange(0L, n)) {
        l[i + 1] = max(l[i], a[i]);
        r[n - i - 1] = max(r[n - i], a[n - i - 1]);
    }

    for (auto i : irange(0L, n)) {
        cout << max(l[i], r[i + 1]) << "\n";
    }
}