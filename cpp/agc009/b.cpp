#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n, -1);
    vector<vector<int64_t>> losers(n);
    for (auto i : irange(1L, n)) {
        cin >> a[i];
        --a[i];
        losers[a[i]].push_back(i);
    }

    vector<int64_t> remains(n);
    for (auto i : irange(0L, n)) {
        remains[i] += losers[i].size();
        if (losers[i].empty()) {
            --remains[a[i]];
        }
    }

    queue<int64_t> q;
    for (auto i : irange(0L, n)) {
        if (remains[i] == 0 && !losers[i].empty()) {
            q.push(i);
        }
    }

    vector<int64_t> d(n, 0);
    while (!q.empty()) {
        auto winner = q.front();
        q.pop();

        vector<pair<int64_t, int64_t>> orders;
        for (auto loser : losers[winner]) {
            orders.emplace_back(d[loser], loser);
        }

        sort(orders.begin(), orders.end());

        d[winner] += orders.size();
        for (auto o : orders | indexed()) {
            d[winner] =
                    max(d[winner], d[o.value().second] +
                                           static_cast<int64_t>(orders.size()) -
                                           o.index());
        }

        if (winner == 0) {
            break;
        }

        --remains[a[winner]];
        if (remains[a[winner]] == 0) {
            q.push(a[winner]);
        }
    }

    cout << d[0] << endl;
}