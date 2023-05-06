#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, l;
    cin >> n >> l;

    vector a(n + 2, 0L), b(n + 2, 0L);
    a[0] = b[0] = 0;
    a[n + 1] = b[n + 1] = l + 1;
    for (auto i : irange(0L, n)) {
        cin >> a[i + 1];
    }
    for (auto i : irange(0L, n)) {
        cin >> b[i + 1];
    }

    vector ok(n + 2, false);

    for (auto i : irange(0L, n + 2)) {
        if (a[i] == b[i]) {
            ok[i] = true;
        }
    }

    int64_t ans = 0;

    multimap<int64_t, int64_t> m;
    for (auto i : irange(0L, n + 2)) {
        if (a[i] != b[i]) {
            m.emplace(b[i] - i, i);
        }

        auto r = m.equal_range(a[i] - i);
        if (r.first != r.second) {
            for (auto it = r.first; it != r.second; ++it) {
                ok[it->second] = true;
            }
            ans += i - r.first->second;
            m.erase(r.first, r.second);
        }
    }

    m.clear();
    for (auto i : irange(0L, n + 2) | reversed) {
        if (a[i] != b[i]) {
            m.emplace(b[i] - i, i);
        }

        auto r = m.equal_range(a[i] - i);
        if (r.first != r.second) {
            for (auto it = r.first; it != r.second; ++it) {
                ok[it->second] = true;
            }
            ans += r.first->second - i;
            m.erase(r.first, r.second);
        }
    }

    if (find(ok.begin(), ok.end(), false) != ok.end()) {
        cout << -1 << endl;
    } else {
        cout << ans << endl;
    }
}