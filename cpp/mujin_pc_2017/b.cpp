#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<vector<bool>> a(n, vector<bool>(n));
    vector<int64_t> r(n);
    for (auto&& aa : a) {
        string s;
        cin >> s;
        for (auto i : irange(0L, n)) {
            aa[i] = (s[i] == '#');
            r[i] += (aa[i] ? 1 : 0);
        }
    }

    if (count(r.begin(), r.end(), 0L) == n) {
        cout << -1 << endl;
        return 0;
    }

    int64_t m = n;
    for (auto i : irange(0L, n)) {
        int64_t k = count(a[i].begin(), a[i].end(), false);

        if (k == 0) {
            m = 0;
        } else if (r[i] > 0) {
            m = min(m, k);
        } else {
            m = min(m, k + 1);
        }
    }

    int64_t ans = n - (count(r.begin(), r.end(), n)) + m;
    cout << ans << endl;
}