#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> xy(n);
    for (auto i : irange(0L, n)) {
        auto&& p = xy[i];
        cin >> p.first >> p.second;
    }

    sort(xy.begin(), xy.end());

    vector<vector<int64_t>> counts(n, vector<int64_t>(n));
}