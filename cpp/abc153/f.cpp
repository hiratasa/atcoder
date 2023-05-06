#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, d, a;
    cin >> n >> d >> a;

    vector<pair<int64_t, int64_t>> xh(n);
    for (auto&& t : xh) {
        cin >> t.first >> t.second;
    }

    sort(xh.begin(), xh.end());

    int64_t ans = 0;
    vector<pair<int64_t, int64_t>> tmp;
    int64_t cur = 0;
    int64_t damage = 0;
    for (auto i : irange(0L, n)) {
        while (cur < tmp.size() && xh[i].first >= tmp[cur].first) {
            damage -= tmp[cur].second;
            ++cur;
        }

        xh[i].second -= damage;
        if (xh[i].second <= 0) {
            continue;
        }

        int64_t m = (xh[i].second - 1) / a + 1;
        ans += m;
        damage += m * a;
        tmp.emplace_back(xh[i].first + 2 * d + 1, m * a);
    }

    cout << ans << endl;
}