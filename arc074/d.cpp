#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(3 * n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> l(n + 1), r(n + 1);

    priority_queue<int64_t> q;
    int64_t s = 0;
    for (auto i : irange(0L, n)) {
        s += a[i];
        q.push(-a[i]);
    }

    l[0] = s;
    for (auto i : irange(n, 2 * n)) {
        if (a[i] > -q.top()) {
            s -= -q.top();
            s += a[i];
            q.pop();
            q.push(-a[i]);
        }

        l[i - n + 1] = s;
    }

    q = priority_queue<int64_t>();
    s = 0;
    for (auto i : irange(2 * n, 3 * n) | reversed) {
        s += a[i];
        q.push(a[i]);
    }

    r[n] = s;
    for (auto i : irange(n, 2 * n) | reversed) {
        if (a[i] < q.top()) {
            s -= q.top();
            s += a[i];
            q.pop();
            q.push(a[i]);
        }

        r[i - n] = s;
    }

    int64_t ans = numeric_limits<int64_t>::min();
    for (auto i : irange(0L, n + 1)) {
        ans = max(ans, l[i] - r[i]);
    }

    cout << ans << endl;
}