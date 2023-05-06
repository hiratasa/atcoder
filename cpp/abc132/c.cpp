#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> d(n);
    for (auto&& dd : d) {
        cin >> dd;
    }

    sort(d.begin(), d.end());

    auto l = d[n / 2 - 1];
    auto r = d[n / 2];

    cout << r - l << endl;
}
