#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<pair<int64_t, int64_t>> p(n);
    for (auto&& ab : p) {
        cin >> ab.first >> ab.second;
    }
    sort(p.begin(), p.end());

    priority_queue<int64_t> q;
    int64_t ans = 0;
    int64_t idx = 0;
    for (int64_t d = m; d >= 0; --d) {
        while (idx < p.size() && d + p[idx].first <= m) {
            q.push(p[idx].second);
            ++idx;
        }

        if (q.empty()) {
            continue;
        }

        ans += q.top();
        q.pop();
    }

    cout << ans << endl;
}