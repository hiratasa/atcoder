#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    sort(a.begin(), a.end());

    auto r = irange(0L, 200002L);
    auto k = *partition_point(r.begin(), r.end(), [&](int64_t k) {
        int64_t num = 0;
        for (auto aa : a) {
            num += a.end() - lower_bound(a.begin(), a.end(), k - aa);
        }

        return num > m;
    });

    vector<int64_t> s(n + 1);
    for (auto i : irange(0L, n)) {
        s[i + 1] = s[i] + a[i];
    }

    int64_t ans = 0, num = 0;
    for (auto aa : a) {
        auto idx = lower_bound(a.begin(), a.end(), k - aa) - a.begin();
        // cerr << aa << "," << (idx < n ? a[idx] : 1000) << "," << idx << endl;
        ans += s[n] - s[idx] + (n - idx) * aa;
        num += n - idx;
    }

    ans += (m - num) * (k - 1);

    cout << ans << endl;
}