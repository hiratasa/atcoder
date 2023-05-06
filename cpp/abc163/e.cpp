#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    priority_queue<pair<int64_t, int64_t>> q;
    for (auto&& aa : a | indexed()) {
        cin >> aa.value();
        q.emplace(aa.value(), aa.index());
    }

    vector dp(n + 1, -1L);
    dp[0] = 0;
    int64_t r = 0;
    while (!q.empty()) {
        auto t = q.top();
        q.pop();

        vector next(n + 1, -1L);
        for (auto i : irange(0L, n)) {
            if (dp[i] < 0) {
                continue;
            }

            next[i + 1] = max(next[i + 1], dp[i] + t.first * abs(t.second - i));
            next[i] = max(next[i],
                          dp[i] + t.first * abs(t.second - (n - r + i - 1)));
        }

        ++r;
        dp = std::move(next);
    }

    cout << *max_element(dp.begin(), dp.end()) << endl;
}