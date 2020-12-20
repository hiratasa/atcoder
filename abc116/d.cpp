#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<pair<int64_t, int64_t>> items(n);
    for (auto&& item : items) {
        int64_t t, d;
        cin >> t >> d;
        item = {d, t};
    }

    sort(items.rbegin(), items.rend());

    vector<bool> used(n, false);
    vector<int64_t> r;
    int64_t score = 0, num_types = 0;
    for (auto i : irange(0L, k)) {
        const auto& item = items[i];

        if (used[item.second]) {
            r.push_back(item.first);
        } else {
            used[item.second] = true;
            ++num_types;
        }

        score += item.first;
    }

    score += num_types * num_types;

    int64_t max_score = score;
    for (auto i : irange(k, n)) {
        if (r.empty()) {
            break;
        }

        const auto& item = items[i];

        if (used[item.second]) {
            continue;
        }
        used[item.second] = true;

        score += item.first;
        score -= r.back();
        r.pop_back();
        score += 2 * num_types + 1;
        ++num_types;
        max_score = max(max_score, score);
    }

    cout << max_score << endl;
}