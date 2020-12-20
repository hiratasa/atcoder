#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n), b(n);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }

    unordered_set<int64_t> q;
    for (auto i : irange(0L, n)) {
        auto prev = (i - 1 + n) % n;
        auto next = (i + 1) % n;
        if (b[i] - (b[prev] + b[next]) >= a[i]) {
            q.insert(i);
        }
    }

    int64_t ans = 0;
    while (!q.empty()) {
        auto idx = *q.begin();
        q.erase(idx);

        auto prev = (idx - 1 + n) % n;
        auto next = (idx + 1) % n;

        auto tmp = (b[idx] - a[idx]) / (b[prev] + b[next]);
        ans += tmp;
        b[idx] -= (b[prev] + b[next]) * tmp;

        auto prev2 = (prev - 1 + n) % n;
        if (b[prev] - (b[prev2] + b[idx]) >= a[prev]) {
            q.insert(prev);
        }

        auto next2 = (next + 1) % n;
        if (b[next] - (b[idx] + b[next2] >= a[next])) {
            q.insert(next);
        }
    }

    if (a == b) {
        cout << ans << endl;
    } else {
        cout << -1 << endl;
    }
}
