#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> a(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i].first;
        a[i].second = i + 1;
    }

    sort(a.begin(), a.end());

    const auto* delim = "";
    for (auto aa : a) {
        cout << delim << aa.second;
        delim = " ";
    }

    cout << endl;
}