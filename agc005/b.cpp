#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> l(n);
    map<int64_t, int64_t> m;
    m[0L] = -1L;
    for (auto i : irange(0L, n)) {
        auto it = --m.upper_bound(a[i]);
        l[i] = it->second;

        ++it;
        m.erase(it, m.end());
        m[a[i]] = i;
    }

    vector<int64_t> r(n);
    m.clear();
    m[0L] = n;
    for (auto i : irange(0L, n) | reversed) {
        auto it = --m.upper_bound(a[i]);
        r[i] = it->second;

        ++it;
        m.erase(it, m.end());
        m[a[i]] = i;
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        // cerr << l[i] << " " << r[i] << endl;
        ans += a[i] * (r[i] - i) * (i - l[i]);
    }

    cout << ans << endl;
}