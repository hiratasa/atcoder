#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

/* 反例
6
80 24 26 160 60 36
200 140 100 40 30 25
*/
main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> ba(n);
    for (auto&& aa : ba) {
        cin >> aa.second;
    }
    for (auto&& aa : ba) {
        cin >> aa.first;
    }

    sort(ba.rbegin(), ba.rend());

    auto cmp = [&](int64_t lhs, int64_t rhs) {
        return make_pair(ba[lhs].second, ba[lhs].first) <
               make_pair(ba[rhs].second, ba[rhs].first);
    };

    priority_queue<int64_t, vector<int64_t>, decltype(cmp)> q(cmp);

    for (auto i : irange(0L, n)) {
        const auto& t = ba[i];
        // a > b
        if (t.second > t.first) {
            q.emplace(i);
        }
    }

    if (q.size() >= n - 1) {
        std::cout << "No" << endl;
        return 0;
    }

    multiset<int64_t> as;
    auto prev_it = ba.begin();
    int64_t nums = 0;
    while (!q.empty()) {
        auto idx = q.top();
        q.pop();
        auto end = partition_point(ba.begin(), ba.end(),
                                   [&](const pair<int64_t, int64_t>& p) {
                                       return ba[idx].second <= p.first;
                                   });

        if (end == ba.begin()) {
            std::cout << "No" << endl;
            return 0;
        }

        for (auto it = prev_it; it != end; ++it) {
            as.emplace(it->second);
        }
        prev_it = end;

        auto it = as.upper_bound(ba[idx].first);
        if (it != as.begin()) {
            --it;
            auto tmp = ba[idx].second;
            ba[idx].second = *it;
            as.erase(it);
            as.emplace(tmp);
        } else if (*it < ba[idx].second) {
            auto tmp = ba[idx].second;
            ba[idx].second = *it;
            as.erase(it);
            as.emplace(tmp);

            q.emplace(idx);
        } else {
            std::cout << "No" << endl;
            return 0;
        }

        ++nums;
        if (nums >= n - 1) {
            std::cout << "No" << endl;
            return 0;
        }
    }

    std::cout << "Yes" << endl;
}