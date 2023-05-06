#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t solve(const vector<int64_t>& x, int64_t l, int first, int last, int prev = -1) {
    static vector<vector<array<int64_t, 2>>> memo(x.size(), vector<array<int64_t, 2>>(x.size(), array<int64_t, 2>{-1, -1}));

    if (first > last) {
        return 0;
    }

    int index = (prev < first ? 0 : 1);

    if (memo[first][last][index] >= 0) {
        cerr << "memo: " << first << " " << last << " " << prev << " " << (index ? 1 : -1) << " " << memo[first][last][index] << endl;
        return memo[first][last][index];
    }

    auto pos = (prev == -1 ? 0 : x[prev]);
    auto dist_first = x[first] - pos;
    if (dist_first < 0) {
        dist_first += l;
    }
    dist_first += solve(x, l, first + 1, last, first);

    auto dist_last = pos - x[last];
    if (dist_last < 0) {
        dist_last += l;
    }
    dist_last += solve(x, l, first, last - 1, last);

    auto ret = max(dist_first, dist_last);
    memo[first][last][index] = ret;

    cerr << first << " " << last << " " << prev << " " << (index ? 1 : -1) << " " << dist_first << " " << dist_last << endl;

    return ret;
}

main() {
    int64_t l, n;
    cin >> l >> n;

    vector<int64_t> x(n);
    for (auto&& xx : x) {
        cin >> xx;
    }

    cout << solve(x, l, 0, n - 1) << endl;
}
