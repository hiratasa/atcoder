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

    vector<int64_t> idx(n);
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](auto lhs, auto rhs) {
        if (a[lhs] != a[rhs]) {
            return a[lhs] < a[rhs];
        } else {
            // 右から
            return lhs > rhs;
        }
    });

    int64_t l = numeric_limits<int64_t>::max();
    set<int64_t> s;
    int64_t ans = 0;
    for (auto ii : idx) {
        auto it = s.lower_bound(ii);
        if (it == s.begin()) {
            ++ans;
            s.insert(ii);
        } else {
            --it;
            s.erase(it);
            s.insert(ii);
        }
    }

    cout << ans << endl;
}