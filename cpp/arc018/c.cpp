#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    int64_t x0, a, p;
    cin >> x0 >> a >> p;

    vector<pair<int64_t, int64_t>> xs(n * m, make_pair(x0, 0L));
    for (auto i : irange(1L, n * m)) {
        xs[i].first = (xs[i - 1].first + a) % p;
        xs[i].second = i;
    }

    sort(xs.begin(), xs.end());

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        vector<int64_t> ms;
        for (auto j : irange(0L, m)) {
            ans += abs(i - xs[i * m + j].second / m);
            ms.push_back(xs[i * m + j].second % m);
        }

        sort(ms.begin(), ms.end());

        for (auto j : irange(0L, m)) {
            ans += abs(j - ms[j]);
        }
    }

    cout << ans << endl;
}