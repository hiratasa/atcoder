#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, c;
    cin >> n >> c;

    vector<pair<int64_t, int64_t>> sushi(n);
    for (auto i : irange(0L, n)) {
        cin >> sushi[i].first >> sushi[i].second;
    }

    vector<int64_t> max_left(n + 1, 0L), max_right(n + 1, 0L);
    int64_t sl = 0, sr = 0;
    for (auto i : irange(0L, n)) {
        sl += sushi[i].second;
        max_left[i + 1] = max(max_left[i], -sushi[i].first + sl);

        sr += sushi[n - i - 1].second;
        max_right[n - i - 1] =
                max(max_right[n - i], -(c - sushi[n - i - 1].first) + sr);
    }

    int64_t ans = max({0L, max_left.back(), max_right.front()});
    sl = 0, sr = 0;
    for (auto i : irange(0L, n)) {
        sl += sushi[i].second;
        auto tl = -2 * sushi[i].first + sl + max_right[i + 1];

        sr += sushi[n - i - 1].second;
        auto tr = -2 * (c - sushi[n - i - 1].first) + sr + max_left[n - i - 1];

        ans = max({ans, tl, tr});
    }

    cout << ans << endl;
}