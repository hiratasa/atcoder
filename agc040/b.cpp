#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<pair<pair<int64_t, int64_t>, int64_t>> lr(n);
    for (auto idx : irange(0L, n)) {
        lr[idx].second = idx;
        auto& t = lr[idx].first;
        cin >> t.first >> t.second;
    }

    if (n == 2) {
        std::cout << (lr[0].first.second - lr[0].first.first + 1) +
                             (lr[1].first.second - lr[1].first.first + 1)
                  << endl;
        return 0;
    }

    int64_t ans = 0;

    auto it1 = max_element(
            lr.begin(), lr.end(), [&](const auto& lr1, const auto& lr2) {
                return make_tuple(lr1.first.first, -lr1.first.second) <
                       make_tuple(lr2.first.first, -lr2.first.second);
            });
    int64_t maxL = it1->first.first;
    int64_t maxL_idx = it1->second;

    auto it2 = min_element(
            lr.begin(), lr.end(), [&](const auto& lr1, const auto& lr2) {
                return make_tuple(lr1.first.second, -lr1.first.first) <
                       make_tuple(lr2.first.second, -lr2.first.first);
            });
    int64_t minR = it2->first.second;
    int64_t minR_idx = it2->second;

    int64_t longest = 0;
    for (const auto& t : lr) {
        if (t.second == maxL_idx || t.second == minR_idx) {
            continue;
        }

        longest = max(longest, t.first.second - t.first.first + 1);
    }

    ans = max(minR - maxL + 1, 0L) + longest;

    sort(lr.rbegin(), lr.rend());

    int64_t currentR = lr[0].first.second;
    for (auto i : irange(1L, n)) {
        ans = max(ans, std::max(currentR - maxL + 1, 0L) +
                               std::max(minR - lr[i].first.first + 1, 0L));
        if (lr[i].first.second == minR) {
            break;
        }
        currentR = min(currentR, lr[i].first.second);
    }

    std::cout << ans << endl;
}