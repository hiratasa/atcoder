#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, q;
    cin >> n >> q;

    vector<int64_t> x(n);
    vector<pair<int64_t, pair<int64_t, int64_t>>> times;
    for (auto i : irange(0L, n)) {
        int64_t s, t, xx;
        cin >> s >> t >> xx;
        x[i] = xx;
        times.emplace_back(s - xx, make_pair(-2L, i));
        times.emplace_back(t - xx, make_pair(-1L, i));
    }

    for (auto i : irange(0L, q)) {
        int64_t d;
        cin >> d;
        times.emplace_back(d, make_pair(0L, i));
    }

    sort(times.begin(), times.end());

    priority_queue<pair<int64_t, int64_t>, vector<pair<int64_t, int64_t>>, greater<pair<int64_t, int64_t>>> queue;
    vector<bool> active(n, false);
    vector<int64_t> ans(q, -1L);
    for (auto&& a : times) {
        auto t = a.first;
        auto type = a.second.first;
        auto idx = a.second.second;
        if (type == -2L) {
            // start
            active[idx] = true;
            queue.emplace(x[idx], idx);
        } else if (type == -1L) {
            // end
            active[idx] = false;
        } else {
            // query
            while (!queue.empty() && !active[queue.top().second]) {
                queue.pop();
            }

            if (!queue.empty()) {
                ans[idx] = queue.top().first;
            }
        }
    }

    for (auto a : ans) {
        cout << a << "\n";
    }

    cout << flush;
}