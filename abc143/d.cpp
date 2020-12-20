#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> l(n);
    for (auto&& ll : l) {
        cin >> ll;
    }

    sort(l.begin(), l.end());

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        auto a = l[i];
        for (auto j : irange(i + 1, n)) {
            auto b = l[j];
            auto it = upper_bound(l.begin() + j + 1, l.end(), b - a);
            auto it2 = lower_bound(l.begin() + j + 1, l.end(), a + b);
            ans += it2 - it;
        }
    }

    cout << ans << endl;
}