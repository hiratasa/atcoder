#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> d(n);
    int64_t s = 0;
    for (auto&& dd : d) {
        cin >> dd;
        s += dd;
    }

    int64_t ans = 0;
    for (auto dd : d) {
        ans += dd * (s - dd);
    }

    ans /= 2;

    cout << ans << endl;
}