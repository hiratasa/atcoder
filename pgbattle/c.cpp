#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> as(n);
    for (auto&& a : as) {
        cin >> a;
    }

    vector<int> index(n);
    iota(index.begin(), index.end(), 0);
    sort(index.begin(), index.end(), [&as](int lhs, int rhs) {
        return as[lhs] < as[rhs];
    });

    vector<int64_t> bs(n);
    int64_t total_score = 0;
    for (auto i : irange(0L, n)) {
        bs[i] = as[index[i]];
        total_score += (i + 1) * bs[i];
    }

    vector<int64_t> cum(n + 1, 0);
    for (auto i : irange(0L, n)) {
        cum[n - i - 1] = cum[n - i] + bs[n - i - 1];
    }

    vector<int64_t> ans(n);
    for (auto i : irange(0L, n)) {
        ans[index[i]] = total_score - cum[i] - i * bs[i];
    }

    for (auto i : irange(0L, n)) {
        cout << ans[i] << endl;
    }

    return 0;
}