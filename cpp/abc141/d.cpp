#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, m;
    cin >> n >> m;

    priority_queue<int64_t> q;
    for (auto _ : irange(0L, n)) {
        int64_t a;
        cin >> a;
        q.push(a);
    }

    for (auto _ : irange(0L, m)) {
        auto p = q.top();
        q.pop();

        q.push(p / 2);
    }

    int64_t ans = 0;
    while (!q.empty()) {
        ans += q.top();
        q.pop();
    }

    cout << ans << endl;
}