#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> times(n);
    for (auto&& time : times) {
        for (auto* t : {&time.first, &time.second}) {
            int32_t h, m, s, ms;
            scanf("%d:%d:%d.%d", &h, &m, &s, &ms);
            h -= 21;
            *t = (3600 * h + 60 * m + s) * 1000 + ms;
        }
    }

    vector<int64_t> ans(n, -1);
    pair<int64_t, int64_t> r{0, 7300000};
    for (auto i : irange(0L, n)) {
        const auto& time = times[i];
        if (time.second <= time.first) {
            ans[i] = time.second - time.first + 1000;
            r.first = max(r.first, time.first);
            r.second = min(r.second, time.second + 1000);
        }
    }

    if (r.second != 7300000) {
        for (auto i : irange(0L, n)) {
            const auto& time = times[i];
            if (ans[i] >= 0) {
                continue;
            }

            if (r.second <= time.first || time.second + 1000 <= r.first) {
                ans[i] = time.second - time.first;
            } else if (!(r.first < time.first + 1000) &&
                       !(time.second < r.second)) {
                ans[i] = time.second - time.first + 1000;
            }
        }
    }

    for (auto a : ans) {
        cout << a << endl;
    }
}