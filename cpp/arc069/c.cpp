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

    map<int64_t, pair<int64_t, pair<int64_t, int64_t>>> m;
    m[0].first = 0;
    for (auto i : irange(0L, n)) {
        auto it = m.upper_bound(a[i]);
        if (it == m.end()) {
            m.rbegin()->second.first = i;
            m[a[i]].first = -1;
            it = --m.end();
        } else {
            --it;
        }
        it->second.second.first += 1;
        it->second.second.second += a[i] - it->first;
    }

    vector<int64_t> ans(n);
    int64_t c = 0, p = 0;
    for (const auto& kv : m | reversed) {
        auto aa = kv.first;
        auto idx = kv.second.first;
        auto ex = kv.second.second.second;

        if (idx >= 0) {
            ans[idx] = c * (p - aa) + ex;
        }

        c += kv.second.second.first;
        p = aa;
    }

    for (auto aa : ans) {
        cout << aa << "\n";
    }
}